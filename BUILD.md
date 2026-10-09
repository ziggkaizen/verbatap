# Build Instructions

This guide covers how to set up the development environment and build VerbaTap from source across different platforms.

## Prerequisites

### All Platforms

- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) package manager
- [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

### Platform-Specific Requirements

#### macOS

- Xcode Command Line Tools
- Install with: `xcode-select --install`

##### Intel Mac (x86_64)

Prebuilt ONNX Runtime binaries are not available for Intel Macs. Install ONNX Runtime via Homebrew and link dynamically:

```bash
brew install onnxruntime
ORT_LIB_LOCATION=$(brew --prefix onnxruntime)/lib ORT_PREFER_DYNAMIC_LINK=1 bun run tauri dev
```

The same environment variables apply for production builds:

```bash
ORT_LIB_LOCATION=$(brew --prefix onnxruntime)/lib ORT_PREFER_DYNAMIC_LINK=1 bun run tauri build
```

#### Windows

- Microsoft C++ Build Tools: Visual Studio 2019/2022 with C++ development
  tools, or Visual Studio Build Tools 2019/2022
- [CMake](https://cmake.org/download/) (must be on `PATH`):

  ```powershell
  winget install Kitware.CMake
  ```

- [Vulkan SDK](https://vulkan.lunarg.com/sdk/home) from LunarG — required to
  build the Vulkan GPU backend (`vulkan-shaders-gen` needs the SDK's headers
  and `glslc`):

  ```powershell
  winget install KhronosGroup.VulkanSDK
  ```

  Open a new terminal afterward so `VULKAN_SDK` is set.

> [!NOTE]
> Windows' 260-character path limit used to break the native Vulkan build in
> most checkouts. Since `transcribe-cpp` 0.1.3 the build works around it
> automatically (it compiles through a short NTFS junction — no admin rights
> or setup needed), so a normal checkout just builds. If you still hit
> path-limit errors, see
> [Windows build fails with path-limit errors](#windows-build-fails-with-path-limit-errors-msb3491--ftk1011--msb6003)
> in Troubleshooting.

#### Linux

- Build essentials
- ALSA development libraries
- Install with:

  ```bash
  # Ubuntu/Debian
  sudo apt update
  sudo apt install build-essential clang libclang-dev libevdev-dev libasound2-dev pkg-config libssl-dev libvulkan-dev vulkan-tools glslc spirv-headers glslang-tools libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libgtk-layer-shell0 libgtk-layer-shell-dev patchelf cmake

  # Fedora/RHEL
  sudo dnf groupinstall "Development Tools"
  sudo dnf install alsa-lib-devel pkgconf openssl-devel vulkan-devel glslc \
    clang clang-devel libevdev-devel \
    spirv-headers-devel spirv-tools-devel glslang \
    gtk3-devel webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel \
    gtk-layer-shell gtk-layer-shell-devel \
    cmake

  # Arch Linux
  sudo pacman -S base-devel clang libevdev shaderc spirv-headers glslang alsa-lib pkgconf openssl vulkan-devel \
    gtk3 webkit2gtk-4.1 libappindicator-gtk3 librsvg gtk-layer-shell \
    cmake
  ```

## Setup Instructions

### 1. Clone the Repository

```bash
git clone git@github.com:ziggkaizen/verbatap.git
cd verbatap
```

### 2. Install Dependencies

```bash
bun install
```

### 3. Start Dev Server

```bash
bun tauri dev
```

### 4. Build for Production

```bash
bun run tauri build
```

This compiles a release binary and generates platform-specific bundles (deb, rpm, AppImage on Linux; dmg on macOS; msi on Windows).

## VibeASR persistent server build proof (Windows x64 only)

This optional build foundation is separate from `bun run tauri build` and Cargo.
It does not bundle a sidecar in VerbaTap, enable an engine, or download models.
The supported proof environment is Windows x64, 64-bit PowerShell 7, Git,
CMake >= 3.24, Ninja, and **MSYS2 UCRT64 GCC**. Install the signed official
[MSYS2 distribution](https://www.msys2.org/) first. From PowerShell, install its
source-backed compiler/runtime packages:

```powershell
& C:\msys64\usr\bin\pacman.exe -Syu --noconfirm
# If MSYS2 asks to restart after updating its core, reopen PowerShell and repeat the update.
& C:\msys64\usr\bin\pacman.exe -S --needed --noconfirm mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-ninja
```

Run the repository-owned build and verification command from the checkout root:

```powershell
pwsh -NoProfile -File .\scripts\build-vibeasr-windows.ps1 -MingwBin C:\msys64\ucrt64\bin -Ninja C:\msys64\ucrt64\bin\ninja.exe
```

The script builds official [VibeASR.cpp](https://github.com/microsoft/VibeASR.cpp/tree/c4334009c88060f86cdbbd684b62662f710b6c20)
at immutable commit `c4334009c88060f86cdbbd684b62662f710b6c20`. Recursive
submodules use the committed gitlinks, including llama.cpp
`a2fdadc20285df2dce90402fca9264a93a8eb32f` and its kompute submodule
`4565194ed7c32d1d2efa32ceab4d3c6cae006306`; no `--remote` or floating branch
selects the build source. Dirty source checkouts are rejected.

At this revision, the upstream persistent target is named `asr_stream_server`.
It compiles `src/asr_server.cpp` and retains models across stdin requests.
The script builds that target and copies its executable as `asr_server.exe`;
it does not change upstream code or use the one-shot `asr_infer` target.
Explicit GCC paths and a separate Ninja build directory isolate this build from
Rust/MSVC. The script restores its process-local `PATH` even on failure and
never writes global compiler, linker, or Cargo configuration.

Outputs under the ignored `target/vibeasr/windows-x64/` directory are:

- `stage/asr_server.exe` and three app-local runtime DLLs:
  `libgcc_s_seh-1.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`;
- `stage/licenses/`: upstream MIT notices, GCC GPLv3/runtime-exception notices,
  and the winpthreads notice from the installed MSYS2 packages;
- `build-manifest.json`: exact source/submodule commits, compiler/CMake/Ninja
  versions, runtime origin directory, and staged-file SHA-256 hashes;
- `runtime-proof.json`: PE import tables, isolated help output, and loader exit
  codes with each DLL withheld and then restored.

llama/ggml are statically linked, OpenMP and curl are disabled, and
`GGML_NATIVE=OFF` avoids selecting the build host's instruction set. The runtime
DLLs come from the same MSYS2 installation as GCC, with published
[package build recipes](https://github.com/msys2/MINGW-packages). This is a
repeatable pinned-source build contract, not a claim of byte-identical outputs
across toolchain versions or inference support on every x64 CPU.

The verifier checks the executable and every staged DLL's x64 PE imports against
the app-local files and an explicit Windows-system allowlist. It actually runs
`asr_server.exe --help` in the staged directory with child `PATH` containing only
`System32` and the Windows directory. Each runtime DLL is temporarily withheld;
the verifier requires `STATUS_DLL_NOT_FOUND` (`0xC0000135`) for each, restores it
in `finally`, then verifies successful launch again. A globally installed runtime
that masks a missing staged DLL therefore fails the proof. No model is loaded.

The non-release `VibeASR Windows x64 proof` workflow repeats this command on
Windows x64 PR/main builds affecting these scripts and uploads the stage and
evidence as a CI proof artifact. Existing app build/release workflows remain
independent. ARM64, macOS, Linux, ASR inference, installation, and production
release are outside this proof.

## Linux Install (from source)

The raw binary (`src-tauri/target/release/verbatap`) cannot run standalone — it needs Tauri resource files (tray icons, sounds, VAD model) to be co-located at the expected path.

**Install from the deb bundle** (works on any Linux distro):

```bash
cd /tmp
ar x /path/to/VerbaTap/src-tauri/target/release/bundle/deb/VerbaTap_*_amd64.deb data.tar.gz
tar xzf data.tar.gz
sudo cp usr/bin/verbatap /usr/bin/
sudo cp -a usr/lib/. /usr/lib/
sudo cp -r usr/share/icons/hicolor/* /usr/share/icons/hicolor/
sudo cp usr/share/applications/VerbaTap.desktop /usr/share/applications/
```

The runtime libraries live in the app-private `/usr/lib/VerbaTap/` (on the binary's rpath), so no `ldconfig` step is needed.

After subsequent rebuilds, copy the binary and any refreshed runtime libraries:

```bash
sudo cp src-tauri/target/release/verbatap /usr/bin/
sudo mkdir -p /usr/lib/VerbaTap
sudo cp -a src-tauri/transcribe-libs/. /usr/lib/VerbaTap/
```

Resources only need re-copying if they change upstream (new icons, sounds, models, etc.).

## Troubleshooting

### macOS Accessibility remains enabled after a local rebuild

Local builds use the ad-hoc `signingIdentity: "-"`. A rebuild can have a new macOS code
identity while the old **System Settings > Privacy & Security > Accessibility** entry
remains visibly enabled, leaving VerbaTap on `Waiting...`.

After installing the final bundle at `/Applications/VerbaTap.app`, quit VerbaTap, clear only its
stale Accessibility record, then reopen it:

```bash
osascript -e 'tell application id "com.ziggkaizen.verbatap" to quit' || true
tccutil reset Accessibility com.ziggkaizen.verbatap
open /Applications/VerbaTap.app
```

Grant Accessibility again when prompted. This does not reset Microphone or other TCC
services, and official releases normally do not need it.

For optional diagnosis, compare the designated requirements of the previous and rebuilt
bundles:

```bash
codesign -dr - /path/to/previous/VerbaTap.app 2>&1
codesign -dr - /Applications/VerbaTap.app 2>&1
```

An ad-hoc requirement contains a `cdhash`; a changed requirement confirms the rebuild is
not covered by the old grant. The reset procedure does not require this check.

See upstream Handy [issue #1618](https://github.com/cjpais/Handy/issues/1618) for the related onboarding
and stale-permission report.

### AppImage build fails on Arch / rolling-release distros

`linuxdeploy` bundles its own `strip` binary which is too old to process system libraries built with newer toolchains on rolling-release distros (Arch, CachyOS, Manjaro, EndeavourOS).

The error from Tauri:

```
Bundling VerbaTap_*_amd64.AppImage
failed to bundle project `failed to run linuxdeploy`
```

Tauri swallows the real linuxdeploy error. To see it, run linuxdeploy manually:

```bash
cd src-tauri/target/release/bundle/appimage
~/.cache/tauri/linuxdeploy-x86_64.AppImage --appimage-extract-and-run \
  --appdir VerbaTap.AppDir --plugin gtk --output appimage
```

**Workaround:** The binary, deb, and rpm bundles all build fine — only the AppImage step fails. To skip it:

```bash
bun run tauri build -- --bundles deb
```

Then install using the deb extraction method above.

### Windows build fails with path-limit errors (`MSB3491` / `FTK1011` / `MSB6003`)

On Windows the native build can fail partway through `transcribe-cpp-sys` with
any of these (all the same root cause):

```
error MSB3491: Could not write lines to file "...VCTargetsPath.tlog\VCTargetsPath.lastbuildstate".
Path: ... exceeds the OS max path limit. The fully qualified file name must be less than 260 characters.
```

```
FileTracker : error FTK1011: could not create the new file tracking log file:
...\vulkan-shaders-gen-build\...\cmTC_xxxxx.tlog\link.write.1.tlog.
The system cannot find the path specified.
```

```
error MSB6003: The specified task executable "CL.exe" could not be run.
System.IO.DirectoryNotFoundException: Could not find a part of the path ...
```

This is **not** a code or toolchain problem — it's Windows' legacy 260-character
path limit (`MAX_PATH`), overflowed by the Vulkan shader generator's nested
CMake build tree on top of Cargo's already-deep
`target\release\build\<crate>-<hash>\out\build\...` directory.

Since `transcribe-cpp` 0.1.3 this is mitigated automatically: the native build
compiles through a short NTFS junction under `%LOCALAPPDATA%\tcs` (created
without admin rights), so a normal checkout builds with no setup. Enabling
Windows long paths does **not** reliably help here — MSBuild's native
`FileTracker` (`tracker.exe`) ignores the long-paths flag — which is why the
junction, not the registry flag, is the fix.

If you still see the errors above, junction creation was likely blocked
(filesystem or corporate policy) — the failing build's log then contains a
`transcribe-cpp-sys: could not create short build junction ...` warning — or
your checkout is deep enough to overflow even the shortened layout. Work
around either case with a short Cargo target directory:

```powershell
# Per-shell:
$env:CARGO_TARGET_DIR = "C:\h"

# Or persist it for all future terminals (note: redirects ALL your
# Rust projects' build output, not just VerbaTap):
[Environment]::SetEnvironmentVariable('CARGO_TARGET_DIR', 'C:\h', 'User')
```

Artifacts then land in `C:\h\release\...` instead of the repo's
`src-tauri\target\`. Open a **new terminal** if you persisted the variable —
it is only picked up by freshly started processes. Then `bun run tauri dev`
and `bun run tauri build` work normally.
