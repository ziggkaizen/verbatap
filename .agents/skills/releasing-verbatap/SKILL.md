---
name: releasing-verbatap
description: Use when a human has explicitly selected a merged VerbaTap state for release and the release candidate, signed multi-platform artifacts, updater metadata, draft GitHub Release, and publish handoff must be verified without conflating build success with release correctness.
---

# Releasing VerbaTap

## Core principle

A VerbaTap release is a separate concern from issue implementation, PR review, merge freshness, and human merge.

Use this skill only after a human has explicitly chosen to prepare/reconcile a release. Do not infer release authorization from a merged PR, a version change, a green main build, or the existence of release automation.

The repository's **current** release workflows/configuration own mutable release mechanics. This skill owns how to reason about and verify the release handoff.

## Authority and stop boundary

```text
MERGED IMPLEMENTATION
!=
RELEASE AUTHORIZATION

SUCCESSFUL RELEASE BUILD
!=
PUBLISHED RELEASE
```

The human owns the final publish/release decision.

Do not autonomously:

- choose a new release version;
- bump version files merely to create a release;
- publish a draft release;
- alter signing/updater policy;
- broaden release scope to fix unrelated product defects.

If those decisions are missing, route them to the appropriate upstream concern instead of inventing them here.

## 1. Establish release identity

Before triggering or judging release work, establish from current evidence:

- intended release version;
- intended source commit/ref;
- current `main` SHA;
- whether the intended source state is merged and current enough for release;
- whether a release/tag/draft for the version already exists;
- current release workflow and build matrix;
- current signing/updater configuration relevant to the release.

Read the current repository sources rather than relying on remembered mechanics, especially:

- `.github/workflows/release.yml`;
- `.github/workflows/build.yml`;
- `src-tauri/tauri.conf.json` and applicable platform overrides;
- current package/version surfaces;
- relevant current release/draft/tag state in GitHub.

If version surfaces disagree, release identity is ambiguous, or the selected source state is not the intended merged state, stop and reconcile before release execution.

## 2. Distinguish evidence classes

Treat these as different properties:

```text
SOURCE STATE SELECTED
VERSION CONSISTENT
TARGET BUILDS
PACKAGE / INSTALL ARTIFACT PRODUCED
BINARY / PACKAGE SIGNED
UPDATER ARTIFACT PRODUCED
UPDATER METADATA CORRECT
ARTIFACT ATTACHED TO INTENDED DRAFT RELEASE
ARTIFACT RUNTIME / INSTALL BEHAVIOR VERIFIED
DRAFT RELEASE COMPLETE
RELEASE PUBLISHED
```

One does not imply the next.

In particular:

```text
GREEN MATRIX
!=
SIGNED ARTIFACTS VERIFIED

SIGNED ARTIFACTS
!=
UPDATER METADATA VERIFIED

DRAFT RELEASE WITH ASSETS
!=
PUBLISHED RELEASE
```

Use `verifying-verbatap-change` for evidence-scope discipline when release work also changes code/configuration or when runtime/platform claims need evaluation.

## 3. Pre-release readiness

Before triggering the release workflow, verify what is materially knowable:

- selected source commit/ref is the intended release source;
- version is intentional and consistent across release-driving surfaces;
- material implementation/review/finalization work for the release state is complete on the intended basis;
- current main/CI/build evidence does not contain a known material blocker relevant to release;
- current release workflow references the expected source/version mechanism;
- signing/updater prerequisites are expected to be available, without exposing secret values;
- no existing tag/release state would make the operation ambiguous or destructive.

Do not treat unavailable secret contents as a defect merely because they cannot be inspected. Verify resulting signed/release behavior from workflow output and artifacts instead.

## 4. Triggering release automation

Only trigger release automation after explicit human authorization for the selected release identity.

After triggering, record the exact workflow run and source state. Do not assume the requested run used the intended ref/version merely because the trigger succeeded.

If automation creates external state such as a draft release, tags, assets, or metadata, read back the resulting state.

## 5. Evaluate the release run

Inspect the actual workflow jobs and failures rather than relying only on the overall workflow badge.

For every current target in the release matrix, classify the result truthfully. Where material, establish:

- target build completed;
- expected package/bundle type was produced;
- target-specific packaging audits/checks passed;
- signing/notarization/code-signing path ran where required;
- updater artifact/signature path ran where required;
- artifact upload attached to the intended draft release.

A target job that never reached signing/upload is not evidence of a signed uploaded artifact even if an earlier compile step succeeded.

## 6. Verify draft release contents

Before calling the release candidate complete, inspect the actual draft GitHub Release and verify as applicable:

- version/tag/name match the intended release identity;
- release remains draft until human publication;
- expected target/package assets are present;
- obvious duplicate, missing, or misnamed assets are surfaced;
- updater artifacts expected by current configuration are present;
- updater metadata points to the correct version/assets and uses the expected current signing model;
- generated release notes are reviewable and do not substitute for human release judgment;
- artifact evidence corresponds to the same release run/source state.

Do not infer updater correctness merely from `createUpdaterArtifacts = true` in configuration. Verify the produced release state.

## 7. Runtime/install evidence

Release-build success proves build/package pipeline behavior, not full end-user runtime behavior.

If the release decision requires install/start/update/runtime confidence beyond existing issue-level evidence, surface that as an explicit release evidence requirement rather than silently upgrading build evidence.

Examples:

```text
MSI PRODUCED + SIGNED
!=
MSI INSTALL / APP START VERIFIED

DMG / APP PRODUCED + NOTARIZATION PATH GREEN
!=
MACOS END-USER PERMISSION FLOW VERIFIED

UPDATER JSON PRESENT
!=
REAL CLIENT UPDATE FLOW VERIFIED
```

Do not invent a universal requirement to manually test every artifact for every release. Scale the evidence to the material release risk and the claims being made.

## 8. Failure routing

Classify failures before choosing the response.

### Release automation / packaging defect

If the selected product state is correct but release mechanics fail, use a focused development issue/correction for the release tooling/configuration. Do not patch unrelated product semantics inside the release operation.

### Product implementation defect

If release evidence exposes a material product/runtime defect, stop the release and route the defect to the earliest owning implementation/design concern. A release workflow is not the place to hide a product fix.

### Verification limitation

If behavior may be correct but release-required evidence is missing, keep the release draft/unpublished and obtain the missing evidence proportionately.

### External/transient infrastructure failure

Distinguish transient runner/service/signing infrastructure problems from repository defects. Retry only when the evidence supports a transient classification; do not repeatedly rerun a deterministic defect.

## 9. Release handoff result

Report:

- intended version;
- selected source commit/ref and current `main` relationship;
- release workflow run identity;
- draft release/tag identity;
- target/package matrix outcome;
- signing/notarization/updater evidence at the scope actually observed;
- missing/failed assets or evidence;
- material runtime/install/update limitations;
- unresolved blockers;
- publication state.

Use one result:

- `READY FOR HUMAN PUBLISH` — the draft release and required evidence are complete on the stated basis;
- `RELEASE RECONCILIATION REQUIRED` — bounded release/tooling/evidence work remains before human publication;
- `RELEASE BLOCKED` — a material product, semantic, authority, signing, release-identity, or unresolved correctness problem prevents safe continuation.

`READY FOR HUMAN PUBLISH` is not permission for the agent to publish. The human owns final publication.
