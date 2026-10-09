#Requires -Version 7.0
<#
.SYNOPSIS
Prove the staged Windows x64 VibeASR dependency closure and isolated launch.
#>
[CmdletBinding()]
param(
    [string]$Stage = (Join-Path $PSScriptRoot '../target/vibeasr/windows-x64/stage'),
    [Parameter(Mandatory)][string]$Objdump
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$Stage = (Resolve-Path -LiteralPath $Stage).Path
$executable = Join-Path $Stage 'asr_server.exe'
if (!(Test-Path -LiteralPath $executable)) { throw "Missing staged server: $executable" }

# An explicit OS allowlist prevents an installed third-party System32 DLL from hiding a missing dependency.
$systemDlls = @('KERNEL32.dll', 'ADVAPI32.dll', 'WS2_32.dll', 'SHELL32.dll',
    'USER32.dll', 'ntdll.dll', 'msvcrt.dll', 'ucrtbase.dll', 'bcrypt.dll')
$imports = [ordered]@{}
$runtimeDlls = @(Get-ChildItem -LiteralPath $Stage -Filter '*.dll')
foreach ($file in @((Get-Item -LiteralPath $executable)) + $runtimeDlls) {
    $headers = & $Objdump -p $file.FullName
    if ($LASTEXITCODE -ne 0) { throw "objdump failed for $($file.Name)" }
    if (($headers -join "`n") -notmatch 'file format pei-x86-64') { throw "Not a Windows x64 PE: $($file.Name)" }
    $dependencies = @($headers | ForEach-Object {
        if ($_ -match 'DLL Name:\s+(\S+)') { $Matches[1] }
    })
    if ($dependencies.Count -eq 0) { throw "No PE imports found for $($file.Name)" }
    $imports[$file.Name] = $dependencies
    foreach ($dependency in $dependencies) {
        if ($dependency -in $systemDlls -or $dependency -like 'api-ms-win-crt-*.dll') { continue }
        if (!(Test-Path -LiteralPath (Join-Path $Stage $dependency))) {
            throw "Missing staged runtime dependency: $dependency (imported by $($file.Name))"
        }
    }
}

function Invoke-IsolatedHelp {
    <# .SYNOPSIS Run the real server with only Windows OS directories on its child PATH. #>
    $start = [System.Diagnostics.ProcessStartInfo]::new()
    $start.FileName = $executable
    $start.ArgumentList.Add('--help')
    $start.WorkingDirectory = $Stage
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true
    $start.RedirectStandardOutput = $true
    $start.RedirectStandardError = $true
    $start.Environment['PATH'] = "$env:SystemRoot\System32;$env:SystemRoot"
    $process = [System.Diagnostics.Process]::new()
    $process.StartInfo = $start
    try {
        [void]$process.Start()
        $stdout = $process.StandardOutput.ReadToEndAsync()
        $stderr = $process.StandardError.ReadToEndAsync()
        if (!$process.WaitForExit(15000)) {
            $process.Kill($true)
            throw 'Isolated --help timed out.'
        }
        return @{ exitCode = $process.ExitCode; output = $stdout.Result + $stderr.Result }
    } finally { $process.Dispose() }
}

# Suppress Windows loader dialogs in the test process and its children; restore the prior mode afterward.
if (!('VibeAsrErrorMode' -as [type])) {
    Add-Type 'using System.Runtime.InteropServices; public static class VibeAsrErrorMode {
        [DllImport("kernel32.dll")] public static extern uint SetErrorMode(uint mode);
    }'
}
$oldMode = [VibeAsrErrorMode]::SetErrorMode(0x8003)
try {
    $positive = Invoke-IsolatedHelp
    if ($positive.exitCode -ne 0 -or $positive.output -notmatch 'Streaming ASR server' -or
        $positive.output -notmatch '---END---') {
        throw "Isolated server --help failed: $($positive.exitCode) $($positive.output)"
    }
    if ($runtimeDlls.Count -eq 0) { throw 'Expected the documented dynamic MinGW runtime posture.' }
    $negative = [ordered]@{}
    foreach ($dll in $runtimeDlls) {
        # Move within the already resolved staging directory only, restoring even on failure.
        $hidden = "$($dll.FullName).withheld"
        if (Test-Path -LiteralPath $hidden) { throw "Unexpected withheld file: $hidden" }
        Move-Item -LiteralPath $dll.FullName -Destination $hidden
        try {
            $result = Invoke-IsolatedHelp
            if ($result.exitCode -ne -1073741515) {
                throw "Expected STATUS_DLL_NOT_FOUND without $($dll.Name), got $($result.exitCode). A global DLL may mask staging."
            }
            $negative[$dll.Name] = $result.exitCode
        } finally { Move-Item -LiteralPath $hidden -Destination $dll.FullName }
    }
    # Ensure every withheld DLL was restored and the final staged artifact remains launchable.
    $restored = Invoke-IsolatedHelp
    if ($restored.exitCode -ne 0) { throw 'Server failed after restoring runtime DLLs.' }
    $evidence = [ordered]@{
        target = 'windows-x86_64'; isolatedPath = "$env:SystemRoot\System32;$env:SystemRoot"
        imports = $imports; helpExitCode = $positive.exitCode; helpOutput = $positive.output
        missingDllExitCodes = $negative; restoredHelpExitCode = $restored.exitCode
    }
    $evidence | ConvertTo-Json -Depth 6 | Set-Content (Join-Path $Stage '../runtime-proof.json')
    Write-Host "Verified isolated --help and $($negative.Count) missing-DLL loader failures."
} finally { [void][VibeAsrErrorMode]::SetErrorMode($oldMode) }
