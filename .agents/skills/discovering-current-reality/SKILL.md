---
name: discovering-current-reality
description: Use when correct VerbaTap work depends on how the current repository, runtime path, platform implementation, compatibility surface, test suite, build system, or external state actually behaves now.
---

# Discovering Current Reality

## Core principle

Establish relevant current state from evidence. Do not design or implement from remembered architecture, historical upstream behavior, tracker state, or green tests.

Discovery is descriptive. It establishes what is true now; it does not choose the target design.

## Start with routing and state

Read applicable `AGENTS.md` first and follow its references to the sources that actually own the relevant fact.

Anchor material observations to the lightest useful state, such as:

- branch/ref or commit SHA;
- PR head/base;
- observation time;
- platform/architecture/target;
- current external-service/dependency state.

Distinguish where relevant:

```text
MERGED CURRENT STATE
UNMERGED / CANDIDATE STATE
LIVE EXTERNAL STATE
```

Do not combine them into one implied reality when they differ.

## Trace the actual behavior path

For user-visible desktop behavior, inspect the real path far enough to locate actual responsibility. Where relevant, trace:

```text
React / UI state
→ Tauri command/event/binding boundary
→ Rust command / coordinator / manager / service
→ platform/native implementation
→ observable desktop behavior
```

Do not stop at a helper merely because its tests are convenient if the production path bypasses, wraps, overrides, or conditionally replaces it.

## VerbaTap discovery surfaces

Inspect only what is material to the change, but consider these categories explicitly when relevant:

### Runtime and native behavior

- audio capture/device lifecycle;
- transcription/model execution;
- clipboard/paste/input behavior;
- global shortcuts;
- overlay/window behavior;
- permissions/accessibility/security constraints;
- CLI and single-instance routing;
- platform-specific `cfg(...)` branches and native dependencies.

### State and compatibility

- persisted settings/defaults;
- migration/legacy parsing;
- environment variables;
- marker strings and public configuration surfaces;
- app-data/resource paths;
- compatibility identifiers inherited from Handy or prior VerbaTap behavior.

### Models, packages, and network behavior

- model catalog metadata and engine identity;
- package roles/files/hashes/paths;
- download/cache/retry/cancel/delete behavior;
- remote endpoints/mirrors;
- whether a path performs actual inference or only prepares metadata/packages;
- optional external post-processing and other network/privacy boundaries.

### Generated and packaged surfaces

- generated TypeScript/Rust bindings;
- generated catalogs/manifests where applicable;
- Tauri/bundle configuration;
- packaged resources;
- installers and target-specific artifacts;
- updater/signing/release configuration when material.

### Verification and CI

Inspect tests and hosted workflows for the exact properties they establish. Identify:

- host operating system;
- architecture/target;
- whether the workflow compiles, packages, unit-tests, browser-tests, or exercises real native runtime behavior;
- path filters that may prevent a workflow from running for the current change.

A successful cross-build does not by itself establish runtime behavior on the target.

## Handy/upstream classification

Before changing inherited names, dependencies, fixtures, comments, markers, or behavior, classify the current role. Useful categories include:

```text
PROVENANCE / ATTRIBUTION
COMPATIBILITY CONTRACT
UPSTREAM / INFRASTRUCTURE DEPENDENCY
TRANSITION MECHANISM
OBSOLETE PRODUCT IDENTITY
ACTUAL DEFECT
```

Do not infer that an inherited reference is wrong from its name alone.

## Conflicting evidence

When sources disagree, state what each source actually establishes and classify the disagreement, for example:

- current vs historical;
- normative vs implemented;
- merged vs candidate;
- shared semantics vs platform-specific mechanics;
- authored source vs generated representation;
- documentation vs runtime.

Resolve from current governing evidence where possible. Otherwise surface the conflict instead of selecting the interpretation that best preserves momentum.

## Completion

Discovery is sufficient when the next concern can proceed without guessing materially relevant current behavior, responsibility location, competing/legacy paths, compatibility constraints, platform differences, or evidence limitations.

Do not keep expanding discovery once remaining uncertainty cannot materially change downstream work.
