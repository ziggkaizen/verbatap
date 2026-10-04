# VerbaTap

**Private, local speech-to-text for Windows, macOS, and Linux.**

VerbaTap is a desktop dictation application. Press a shortcut, speak, and have the transcription inserted into the application you are using.

Speech transcription runs locally on your computer. Optional post-processing features may contact an external provider only when explicitly configured and enabled.

## Status

VerbaTap is under active development.

The project began as a fork of [Handy](https://github.com/cjpais/Handy) and is being separated into its own product identity, runtime, packaging, documentation, and visual assets.

Automatic update checks are disabled by default while VerbaTap's own release-signing infrastructure is being configured.

## Features

- Local speech-to-text
- Windows, macOS, and Linux support
- Configurable global dictation shortcuts
- Hold, toggle, and automatic shortcut behavior
- System tray integration
- Recording and transcription overlay
- Transcription history
- Multiple local speech-recognition models
- Custom vocabulary support
- Optional post-processing
- CLI control for automation and desktop integrations

## Installation

VerbaTap builds are published through the project's GitHub Releases page when available:

https://github.com/ziggkaizen/verbatap/releases

For Debian/Ubuntu packages, install the downloaded `.deb` using APT:

```bash
sudo apt install ./VerbaTap_*.deb
```

Windows and macOS packages should be installed from the matching VerbaTap release artifact.

Homebrew and winget packages for Handy are not VerbaTap packages and should not be used to install VerbaTap.

## Development

See [BUILD.md](BUILD.md) for platform-specific prerequisites and build instructions.

Basic setup:

```bash
git clone https://github.com/ziggkaizen/verbatap.git
cd verbatap
bun install
bun run tauri dev
```

Frontend verification:

```bash
bun run build
```

Rust verification:

```bash
cargo check --manifest-path ./src-tauri/Cargo.toml
```

## CLI

VerbaTap supports command-line control of both startup behavior and a running instance.

```bash
verbatap --toggle-transcription
verbatap --toggle-post-process
verbatap --cancel

verbatap --start-hidden
verbatap --no-tray
verbatap --debug
verbatap --help
```

On macOS, when installed as an application bundle:

```bash
/Applications/VerbaTap.app/Contents/MacOS/verbatap --toggle-transcription
```

On Linux, transcription can also be toggled with:

```bash
pkill -USR2 -n verbatap
```

## App Data

Typical application-data locations are:

```text
macOS:   ~/Library/Application Support/com.ziggkaizen.verbatap/
Windows: %APPDATA%\com.ziggkaizen.verbatap\
Linux:   ~/.config/com.ziggkaizen.verbatap/
```

The actual path can also be viewed from Settings > About.

## Models

VerbaTap supports local model downloads and models already available on disk or in supported shared caches.

Some model files are currently still downloaded from upstream Handy infrastructure such as `blob.handy.computer`. Those URLs are retained as infrastructure dependencies and do not represent VerbaTap branding or affiliation.

## Linux Notes

Linux text insertion depends on the display server and installed input tools. Common choices include `xdotool` on X11 and `wtype`, `dotool`, or `ydotool` on Wayland.

VerbaTap also uses `gtk-layer-shell` for overlay integration on supported Linux environments.

Some inherited compatibility environment variables still use the `HANDY_*` prefix. These identifiers are intentionally preserved until a migration strategy is introduced.

## macOS Secure Input

macOS Secure Input can temporarily prevent global keyboard shortcuts from reaching VerbaTap. This commonly happens while a password field is active or when Terminal has **Secure Keyboard Entry** enabled.

If VerbaTap reports that shortcuts are blocked, leave password fields, disable Secure Keyboard Entry in Terminal if enabled, and retry the shortcut. If macOS continues to report Secure Input after the responsible application has closed, logging out or restarting macOS can clear the stale state.

## Reporting Issues

Please report VerbaTap problems in this repository:

https://github.com/ziggkaizen/verbatap/issues

Include the VerbaTap version, operating system, relevant hardware, steps to reproduce, and logs when useful. Remove dictated text, API keys, personal file paths, or other private information before posting logs.

## Upstream and Attribution

VerbaTap is an independent fork of [Handy](https://github.com/cjpais/Handy), originally created by CJ Pais and contributors.

The inherited source code is distributed under the MIT License. The original copyright and license notice are preserved in [LICENSE](LICENSE).

The Handy name, logo, icon, and other Handy brand assets are not licensed for reuse. VerbaTap uses its own product name and separate VerbaTap-specific visual assets.

VerbaTap is not affiliated with or endorsed by Handy or its maintainers.

## License

MIT License. See [LICENSE](LICENSE).