# Agent Instructions

## Development workflow

VerbaTap uses repository-local workflow skills in `.agents/skills/`.

Use the **earliest unresolved material concern** that could change downstream work. Apply only the rigor needed for the actual risk. If unsure, use `using-development-workflow`.

## Authority boundaries

Keep these roles separate:

- **VerbaTap ChatGPT Project Sources `00–08`** own ChatGPT-side workflow coordination, planning concerns, and stable project orientation.
- **`AGENTS.md` and `.agents/skills/`** own repository-local execution, verification, review, handoff, and finalization mechanics.
- **The current repository, runtime, CI, GitHub state, and external systems** own mutable current facts.
- **The GitHub development issue** owns task semantics for one bounded implementation unit.
- **The pull request** owns the reviewable implementation candidate and handoff evidence.
- **The human operator** owns final merge and release/publish decisions.

Do not duplicate authority between layers. If implementation requires inventing missing intent, architecture, semantics, scope, or prerequisites, stop and route the missing concern upstream.

## Repository-local routing

| Need | Skill |
|---|---|
| Unsure which repo-local concern applies | `using-development-workflow` |
| Material current-system facts are unknown | `discovering-current-reality` |
| One ready issue is being implemented | `executing-one-issue` |
| Verification scope/evidence is unclear | `verifying-verbatap-change` |
| A completed delivery needs independent review | `reviewing-issue-implementation` |
| A reviewed PR needs final freshness before merge | `finalizing-reviewed-pr` |
| A merged state is explicitly being prepared for release | `releasing-verbatap` |
| Real usage has produced workflow-learning evidence | `evaluating-workflow-use` |

Framing, alignment, target design, specification, decomposition, and issue authoring remain Project Source concerns unless a future repository skill explicitly owns them.

## Core execution rules

For substantial implementation work:

```text
ONE BOUNDED ISSUE
→ ONE FRESH IMPLEMENTATION CONTEXT
→ ONE FOCUSED PR
→ INDEPENDENT REVIEW
→ FINAL MERGE FRESHNESS
→ HUMAN MERGE
```

- The issue owns task semantics; skills govern execution and evidence.
- Read the complete original issue before work and re-read it before completion.
- Use acceptance-criteria checkboxes as active progress state. Tick only outcomes actually established by evidence.
- Green tests prove only the properties and paths they exercise.
- Scope platform/architecture/target claims to the evidence actually obtained.
- Handle bounded repository drift with proportional reconciliation. Use Controlled Re-Entry only for upstream-owned semantic/architectural/scope/prerequisite/responsibility changes.
- Review approval is snapshot-bound to the reviewed PR head and relevant base/current-main state.
- After verified PR handoff, stop. Do not merge, manually close the issue, start the next substantial issue, trigger a release, or perform the independent review from the implementation context.

## VerbaTap guardrails

- **Local-first/privacy:** do not introduce external/network processing, upload, telemetry, or remote-provider behavior implicitly.
- **Cross-platform:** target build success is not target runtime proof; one-platform evidence is not cross-platform verification.
- **Compatibility:** do not rename persisted values, legacy markers, environment variables, public configuration, app-data semantics, or other compatibility-sensitive identifiers without establishing migration/compatibility requirements.
- **Handy/upstream:** inherited `Handy`, `HANDY_*`, `cjpais`, or similar references are not automatically defects. Classify provenance, compatibility, dependency, transition, obsolete identity, or actual defect before changing them.
- **Generated/package surfaces:** determine the real role of generated files, packaged resources, installers, updater/signing configuration, and release artifacts when affected.

Use `discovering-current-reality` and `verifying-verbatap-change` for the detailed mechanics behind these rules.

## Current-source routing

Read current repository sources instead of freezing mutable mechanics here:

- `CONTRIBUTING.md` — contributor expectations;
- `BUILD.md` — platform build prerequisites/guidance;
- `package.json` — frontend/tooling scripts;
- `src-tauri/Cargo.toml` — Rust dependencies/features/targets;
- `.github/workflows/` — hosted CI/build/release mechanics and target matrices;
- current source/tests/configuration — actual implementation/runtime behavior.

`AGENTS.md` routes; it does not replace these sources.

## GitHub handoff

Before creating/updating an issue or PR, read the **current** applicable template. Templates structure handoff; they do not replace issue semantics or verification requirements.

Never fabricate human-authored testimony. After material GitHub mutations, read back resulting state when correctness depends on the mutation landing as intended.

## Release boundary

Ordinary implementation stops at verified PR handoff, and human merge does not automatically authorize release. Use `releasing-verbatap` only after explicit human release authorization. The human owns final publication.
