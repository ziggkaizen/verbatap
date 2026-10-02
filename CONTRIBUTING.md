# Contributing to VerbaTap

Thanks for contributing to VerbaTap.

The project values focused changes, simple implementations, local-first behavior, and compatibility across Windows, macOS, and Linux.

## Development Setup

Clone VerbaTap:

```bash
git clone git@github.com:ziggkaizen/verbatap.git
cd verbatap
bun install
```

Run the application:

```bash
bun run tauri dev
```

See [BUILD.md](BUILD.md) for platform-specific prerequisites.

The original Handy repository can optionally be added as an upstream reference:

```bash
git remote add upstream git@github.com:cjpais/Handy.git
```

Do not merge upstream changes blindly. VerbaTap intentionally has its own identity, packaging, release configuration, and product direction.

## Issues

Search existing VerbaTap issues before opening a duplicate:

https://github.com/ziggkaizen/verbatap/issues

Bug reports should include reproduction steps, operating system, VerbaTap version, relevant hardware, and logs when useful.

Remove private dictated text, secrets, API keys, or personal paths from logs before posting them.

## Changes and Pull Requests

Keep each pull request focused on one coherent problem or feature.

Follow the existing architecture and avoid unnecessary abstractions or migrations.

Do not rename compatibility identifiers such as persisted settings values, legacy marker strings, environment variables, or public configuration interfaces unless the change includes an explicit migration strategy.

Use the repository pull-request template and describe how the change was verified.

## Verification

At minimum, run the checks relevant to the files you changed.

Frontend:

```bash
bun run build
```

Rust:

```bash
cargo check --manifest-path ./src-tauri/Cargo.toml
```

Whitespace:

```bash
git diff --check
```

Run focused tests for the changed behavior as appropriate.

Changes affecting installers, packaging, or platform-specific behavior should also be verified through the relevant CI build matrix.

## Code Style

Rust code should follow standard formatting and existing repository patterns.

TypeScript and React code should remain strongly typed and follow the existing component and hook structure.

Prefer straightforward code over clever abstractions.

## AI Assistance

AI-assisted contributions are allowed.

If AI tools materially contributed to a pull request, disclose that in the PR template and verify the resulting changes yourself.

AI-generated review findings should be treated as untrusted until confirmed against the current code.

## Translations

See [CONTRIBUTING_TRANSLATIONS.md](CONTRIBUTING_TRANSLATIONS.md).

Do not translate the VerbaTap brand name.

## Upstream Attribution

VerbaTap is based on Handy by CJ Pais and contributors.

Keep upstream historical references when they explain inherited behavior, fixes, compatibility constraints, or source provenance.

Do not reuse the Handy name, logo, icon, or other Handy brand assets as VerbaTap branding.

## License

Contributions to VerbaTap are made under the project's MIT License. See [LICENSE](LICENSE).