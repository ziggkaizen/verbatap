#Requires -Version 7.0
<#
.SYNOPSIS
Build and prove the pinned persistent VibeASR server using MSYS2 UCRT64 GCC.
.DESCRIPTION
Opt-in Windows x64 build only. Outputs stay under target/vibeasr and do not
change Cargo, Tauri, global PATH, or production packaging.
#>
[CmdletBinding()]
param(
    [string]$MingwBin = 'C:\msys64\ucrt64\bin',
    [string]$Ninja = 'ninja.exe',
    [ValidateRange(1, 64)][int]$Jobs = 4
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if (![OperatingSystem]::IsWindows() -or
    [Runtime.InteropServices.RuntimeInformation]::OSArchitecture -ne 'X64' -or
    [Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture -ne 'X64') {
    throw 'This proof requires Windows x64 and 64-bit PowerShell 7.'
}
$upstreamUrl = 'https://github.com/microsoft/VibeASR.cpp.git'
$upstreamCommit = 'c4334009c88060f86cdbbd684b62662f710b6c20'
$upstreamTarget = 'asr_stream_server'
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$outputRoot = Join-Path $repoRoot 'target/vibeasr'
$source = Join-Path $outputRoot 'source'
$build = Join-Path $outputRoot 'windows-x64/build'
$stage = Join-Path $outputRoot 'windows-x64/stage'
$runtimeNames = @('libgcc_s_seh-1.dll', 'libstdc++-6.dll', 'libwinpthread-1.dll')

function Invoke-BuildTool {
    <# .SYNOPSIS Run one native build command and stop on its exit status. #>
    param([string]$Tool, [string[]]$Arguments)
    & $Tool @Arguments | Out-Host
    if ($LASTEXITCODE -ne 0) { throw "$Tool failed with exit code $LASTEXITCODE" }
}

# Fail before source acquisition if prerequisites are missing; never fall back to MSVC.
foreach ($name in @('gcc.exe', 'g++.exe', 'objdump.exe') + $runtimeNames) {
    if (!(Test-Path -LiteralPath (Join-Path $MingwBin $name))) {
        throw "Missing $name in $MingwBin. Install MSYS2 mingw-w64-ucrt-x86_64-gcc and pass -MingwBin."
    }
}
$MingwBin = (Resolve-Path -LiteralPath $MingwBin).Path
$gcc = Join-Path $MingwBin 'gcc.exe'
$gxx = Join-Path $MingwBin 'g++.exe'
$objdump = Join-Path $MingwBin 'objdump.exe'
foreach ($tool in @('git.exe', 'cmake.exe', $Ninja)) {
    if (!(Get-Command $tool -CommandType Application -ErrorAction SilentlyContinue)) {
        throw "Missing $tool. Install Git, CMake >= 3.24 and Ninja; use -Ninja for an absolute Ninja path."
    }
}
$Ninja = (Get-Command $Ninja -CommandType Application).Source
$licenseRoot = [IO.Path]::GetFullPath((Join-Path $MingwBin '../share/licenses'))
# MSYS2 split the runtime packages in GCC 16; retain the installed GCC 13 layout too.
$gccLicensePackages = @('libgcc', 'libstdc++')
if (!(Test-Path -LiteralPath (Join-Path $licenseRoot 'libgcc'))) { $gccLicensePackages = @('gcc-libs') }
$pthreadPackage = if (Test-Path -LiteralPath (Join-Path $licenseRoot 'libwinpthread')) { 'libwinpthread' } else { 'winpthreads' }
$runtimeLicenses = @(
    foreach ($package in $gccLicensePackages) { "$package/COPYING3"; "$package/COPYING.RUNTIME" }
    "$pthreadPackage/COPYING"
)
foreach ($license in $runtimeLicenses) {
    if (!(Test-Path -LiteralPath (Join-Path $licenseRoot $license))) {
        throw "Missing MSYS2 runtime license: $licenseRoot/$license. Use the documented UCRT64 packages."
    }
}

$originalPath = $env:PATH
try {
    # GCC's own subprocesses need its DLLs; this process-local PATH is restored before returning.
    $env:PATH = "$MingwBin;$originalPath"
    foreach ($compiler in @($gcc, $gxx)) {
        $machine = & $compiler -dumpmachine
        if ($LASTEXITCODE -ne 0 -or $machine -ne 'x86_64-w64-mingw32') {
            throw "Expected x86_64-w64-mingw32 GCC, got $machine from $compiler"
        }
    }
    $compilerVersion = (& $gcc --version | Select-Object -First 1)
    New-Item -ItemType Directory -Force $outputRoot | Out-Null
    if (!(Test-Path -LiteralPath $source)) {
        Invoke-BuildTool git.exe @('clone', '--no-checkout', $upstreamUrl, $source)
        # A no-checkout clone has no populated index yet; establish the pin before checking cleanliness.
        Invoke-BuildTool git.exe @('-C', $source, 'checkout', '--detach', $upstreamCommit)
    }
    $actualOrigin = & git -C $source remote get-url origin
    if ($LASTEXITCODE -ne 0 -or $actualOrigin -ne $upstreamUrl) { throw 'Unexpected VibeASR source origin.' }
    $dirty = & git -C $source status --porcelain --untracked-files=all
    if ($LASTEXITCODE -ne 0 -or $dirty) { throw 'VibeASR source has local changes; use a clean source checkout.' }
    Invoke-BuildTool git.exe @('-C', $source, 'checkout', '--detach', $upstreamCommit)
    # No --remote: the superproject's gitlinks, including recursive ones, own dependency revisions.
    Invoke-BuildTool git.exe @('-C', $source, 'submodule', 'update', '--init', '--recursive')
    $actualCommit = & git -C $source rev-parse HEAD
    if ($LASTEXITCODE -ne 0 -or $actualCommit -ne $upstreamCommit) { throw 'VibeASR source pin mismatch.' }
    $submodules = @(& git -C $source submodule status --recursive)
    if ($LASTEXITCODE -ne 0 -or @($submodules | Where-Object { $_ -notmatch '^ [0-9a-f]{40} ' }).Count) {
        throw 'Recursive VibeASR submodules do not match pinned gitlinks.'
    }
    Invoke-BuildTool git.exe @('-C', $source, 'submodule', 'foreach', '--recursive',
        'test -z "$(git status --porcelain --untracked-files=all)"')
    Invoke-BuildTool cmake.exe @('--fresh', '-S', $source, '-B', $build, '-G', 'Ninja',
        "-DCMAKE_MAKE_PROGRAM=$Ninja", "-DCMAKE_C_COMPILER=$gcc", "-DCMAKE_CXX_COMPILER=$gxx",
        '-DCMAKE_BUILD_TYPE=Release', '-DBUILD_SHARED_LIBS=OFF', '-DLLAMA_BUILD_COMMON=ON',
        '-DGGML_NATIVE=OFF', '-DGGML_OPENMP=OFF', '-DLLAMA_CURL=OFF')
    Invoke-BuildTool cmake.exe @('--build', $build, '--target', $upstreamTarget, '--parallel', "$Jobs")

    # Recreate only the fixed output directory, after verifying its absolute workspace boundary.
    $resolvedStage = [IO.Path]::GetFullPath($stage)
    if (!$resolvedStage.StartsWith("$repoRoot\target\vibeasr\", [StringComparison]::OrdinalIgnoreCase)) {
        throw "Unsafe staging path: $resolvedStage"
    }
    if (Test-Path -LiteralPath $resolvedStage) { Remove-Item -LiteralPath $resolvedStage -Recurse -Force }
    New-Item -ItemType Directory -Force $resolvedStage | Out-Null
    Copy-Item -LiteralPath (Join-Path $build "bin/$upstreamTarget.exe") -Destination (Join-Path $stage 'asr_server.exe')
    foreach ($name in $runtimeNames) {
        Copy-Item -LiteralPath (Join-Path $MingwBin $name) -Destination $stage
    }
    $notices = Join-Path $stage 'licenses'
    New-Item -ItemType Directory -Force $notices | Out-Null
    Copy-Item -LiteralPath (Join-Path $source 'LICENSE') -Destination (Join-Path $notices 'VibeASR-MIT.txt')
    Copy-Item -LiteralPath (Join-Path $source '3rdparty/llama.cpp/LICENSE') -Destination (Join-Path $notices 'llama-ggml-MIT.txt')
    foreach ($license in $runtimeLicenses) {
        $noticeName = $license.Replace('/', '-') + '.txt'
        Copy-Item -LiteralPath (Join-Path $licenseRoot $license) -Destination (Join-Path $notices $noticeName)
    }
    & (Join-Path $PSScriptRoot 'test-vibeasr-runtime.ps1') -Stage $stage -Objdump $objdump
    $files = @(Get-ChildItem -LiteralPath $stage -File -Recurse | ForEach-Object {
        @{ path = [IO.Path]::GetRelativePath($stage, $_.FullName); sha256 = (Get-FileHash -LiteralPath $_.FullName).Hash }
    })
    [ordered]@{
        upstreamUrl = $upstreamUrl; upstreamCommit = $actualCommit; upstreamTarget = $upstreamTarget
        submodules = $submodules; compiler = $compilerVersion; compilerPath = $gcc
        runtimeSource = $MingwBin; cmake = (& cmake.exe --version | Select-Object -First 1)
        ninja = (& $Ninja --version); files = $files
    } | ConvertTo-Json -Depth 5 | Set-Content (Join-Path $stage '../build-manifest.json')
    Write-Host "Verified Windows x64 persistent server staged at $stage"
} finally { $env:PATH = $originalPath }
