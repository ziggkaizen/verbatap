---
name: verifying-verbatap-change
description: Use to choose and evaluate proportionate verification for a VerbaTap change across frontend, Rust, Tauri boundaries, native platforms, persistence, models, packaging, and release-adjacent surfaces without overstating evidence.
---

# Verifying VerbaTap Change

## Core principle

Verify the **properties the change claims**, on the **paths and platforms that matter**, with evidence strong enough for those claims.

There is intentionally no universal "run one command and VerbaTap is verified" rule.

A command succeeding, CI being green, or a target compiling is not stronger evidence than the behavior actually exercised.

## 1. Classify the changed surface and claim scope

Before selecting checks, identify which surfaces are materially affected:

- documentation/configuration only;
- React/TypeScript frontend;
- Tauri commands/events/generated bindings;
- Rust application/core behavior;
- settings/persistence/migration/compatibility;
- localization;
- model catalog/package/download/cache behavior;
- audio/transcription/inference behavior;
- CLI/single-instance behavior;
- clipboard/paste/input/shortcut/overlay/native integration;
- platform/architecture-specific behavior;
- packaging/install/resources;
- signing/updater/release behavior;
- privacy/network boundary.

Then identify the claim scope:

```text
SOURCE / TYPE CORRECTNESS
UNIT / PROPERTY BEHAVIOR
INTEGRATED APPLICATION PATH
PLATFORM RUNTIME BEHAVIOR
TARGET BUILDABILITY
PACKAGE / INSTALLABILITY
CROSS-PLATFORM BEHAVIOR
RELEASE / SIGNING BEHAVIOR
```

Choose evidence for the actual claim rather than automatically running the largest suite.

## 2. Re-read current verification mechanics

Before executing verification, inspect current repository-owned mechanics because commands and workflows may change:

- `package.json` scripts;
- `src-tauri/Cargo.toml` target/features;
- `.github/workflows/`;
- `BUILD.md` when platform build prerequisites matter.

Current common entry points include, when still present and relevant:

```text
bun run lint
bun run build
bun run test:playwright
bun run check:translations
bun run check:model-languages
bun run format:check

git diff --check

cd src-tauri
cargo fmt -- --check
cargo check
cargo test
cargo clippy
```

Do not treat this list as an obligation to run every command for every change. Do not treat a historical/pre-existing whole-repository failure as a new defect without establishing the delta/baseline relationship.

## 3. Baseline evidence by surface

### Documentation / non-executable metadata

Usually verify:

- content against the governing source/intent;
- links/paths/structured syntax where relevant;
- `git diff --check`;
- any focused parser/schema check the file participates in.

Do not require unrelated application builds merely because they exist.

### React / TypeScript frontend

Usually consider:

- `bun run lint`;
- `bun run build` for TypeScript + Vite integration;
- focused tests where present;
- Playwright when the changed property is represented in the browser-testable UI path;
- visual/manual evidence when appearance or interaction quality cannot be established by automated tests.

Playwright in this repository is browser evidence. It does **not** automatically prove Tauri/native desktop integration.

### Rust application/core behavior

Usually consider:

- `cargo fmt -- --check`;
- `cargo check`;
- focused regression/property tests;
- `cargo test` when the broader Rust suite can materially catch regressions;
- `cargo clippy` when production Rust changes or lint-sensitive correctness/quality make it material.

A Linux-hosted `cargo test` run proves only what those tests exercise under that build/runtime configuration.

### Tauri boundary / generated bindings

When commands, events, shared payloads, or generated bindings change, verify both sides of the boundary:

```text
frontend caller/consumer
→ generated/shared representation
→ Tauri command/event
→ Rust implementation
```

Use frontend and Rust checks as appropriate, regenerate bindings/artifacts when required, inspect the generated delta, and exercise the real boundary when the required property depends on integration rather than type shape alone.

### Settings / persistence / compatibility

Where material, verify:

- defaults;
- read/write round trip;
- existing persisted representation;
- legacy/compatibility parsing;
- migration/cutover behavior;
- invalid/partial state handling;
- restart/persistence behavior when relevant.

A test of the new representation alone does not prove compatibility with existing state.

### Localization

Where user-facing text or language metadata changes, consider:

- `bun run check:translations`;
- `bun run lint` for i18n rules;
- actual UI rendering/overflow/pluralization/directionality only when materially affected.

### Model catalog / packages / download / cache

Separate metadata/package readiness from actual inference support.

Where material, verify properties such as:

- engine/model identity;
- package roles and deterministic paths;
- size/hash metadata;
- catalog/language consistency;
- cache/readiness state;
- download retry/resume/cancel/mirror/delete behavior;
- partial/corrupt package handling;
- preservation of a currently working engine/model when a new unsupported path fails.

Do not claim inference/runtime support merely because package metadata or downloads work.

### Audio / transcription / inference

Use focused unit/property tests for deterministic logic, but obtain actual runtime evidence when the issue requires properties that depend on real devices, native audio, model loading, acceleration backend, or end-to-end transcription.

Record relevant platform, architecture, device/backend, and model when those facts materially bound the evidence.

### CLI / single instance

Parsing tests prove parsing. They do not automatically prove second-instance routing to an already-running application.

When that runtime path changes materially, verify the actual single-instance/control flow on a relevant platform where practical.

### Clipboard / paste / input / shortcuts / overlay / permissions

These are native integration surfaces. Where behavior changes materially, unit tests of helpers are supplementary; verify the actual native path, relevant permissions, fallback behavior, and failure behavior on affected platforms where practical.

### Platform / architecture-specific behavior

For code behind platform/architecture conditionals, identify every platform/target the change claims to support.

Use the narrowest sufficient combination of:

- native-host tests;
- target compilation;
- full package build;
- actual runtime/manual evidence;
- relevant hosted CI/build matrix.

Never equate target compilation/package creation with runtime behavior verification.

### Packaging / installers / resources

When package contents, resources, Tauri configuration, installer behavior, or target-specific build logic changes, use the relevant build workflow/target and inspect the produced artifact where the issue requires artifact correctness.

The current `pr-test-build.yml` can provide cross-target PR build artifacts when appropriate. Its success establishes build/package production for those configured targets, not necessarily installation/runtime correctness on each target.

### Signing / updater / release

Ordinary implementation verification must not silently upgrade build evidence into release evidence.

When signing, updater metadata, release assets, or publishing logic changes, explicitly identify what was and was not exercised. A dedicated release workflow/human-controlled release step may still be required.

### Privacy / network boundary

When network behavior changes, verify the intended trigger and data flow:

```text
DEFAULT / LOCAL PATH
→ no unintended external transfer

EXPLICITLY ENABLED REMOTE PATH
→ intended provider/data/request behavior only
```

Where material, test disabled/default behavior as well as enabled behavior.

## 4. Hosted workflow interpretation

Interpret hosted evidence precisely.

At the current repository state:

- `test.yml` runs Rust tests on Ubuntu for `src-tauri/**` changes;
- `code-quality.yml` runs translation/model-language checks, ESLint, and Prettier on selected frontend/config paths;
- `playwright.yml` runs Chromium Playwright for selected frontend/test paths;
- `main-build.yml` performs the current full cross-platform build matrix on `main`;
- `pr-test-build.yml` manually builds the configured PR merge ref across the current target matrix;
- `release.yml` creates a draft release and invokes signed release builds.

Re-check these files before relying on this description. Path filters and workflow changes can make historical assumptions stale.

## 5. Evidence status

For each material required property use one truthful state:

- `VERIFIED` — evidence establishes the property at the claimed scope;
- `NOT APPLICABLE` — the property/check does not apply to this change;
- `NOT AVAILABLE` — suitable evidence cannot currently be obtained in this context/environment;
- `NOT VERIFIED` — applicable property remains unestablished.

`NOT AVAILABLE` and `NOT VERIFIED` are evidence limitations, not alternate spellings of `VERIFIED`.

## 6. Failure and negative behavior

For required failure/prohibition behavior, establish when material:

```text
VALID PRECONDITIONS
→ INTENDED BRANCH / PATH REACHED
→ INTENDED FAILURE / PROHIBITION OBSERVED
→ NO FORBIDDEN PARTIAL STATE / SIDE EFFECT
```

Do not accept a failing test as proof of the intended failure if it fails earlier for an unrelated reason.

## 7. Completion report

Verification reporting should state:

- changed/claimed surfaces;
- commands/tests/manual/CI evidence actually used;
- platform/architecture/target where material;
- properties established;
- material evidence limitations;
- applicable properties still `NOT VERIFIED` or `NOT AVAILABLE`.

Use concise evidence language. Do not turn verification into a command transcript when the important fact is what each result establishes.
