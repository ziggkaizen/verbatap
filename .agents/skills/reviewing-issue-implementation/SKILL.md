---
name: reviewing-issue-implementation
description: Use when a completed VerbaTap issue delivery needs an independent check against original issue intent, governing repository sources, actual implementation/runtime paths, tests/evidence, platform scope, and handoff state.
---

# Reviewing Issue Implementation

## Core principle

Evaluate the delivered issue independently. The implementer's completion claim, checked acceptance criteria, green tests, successful build, and AI review comments are inputs, not proof.

Prefer a fresh reviewer context that did not participate in implementation reasoning.

## Establish the reviewed candidate

Before judging correctness:

- verify the actual issue and pull request;
- read the complete original issue and referenced controlling sources;
- identify current PR head SHA;
- identify relevant base/current `main` SHA;
- distinguish merged repository state from the unmerged candidate;
- account for materially relevant recent/parallel work;
- identify the platforms/architectures/targets the implementation actually claims to support.

Do not adopt issue numbers, PR identity, completion claims, platform claims, or checked boxes without verification when they materially identify the work.

## Review order

```text
WORK IDENTITY / REPOSITORY STATE
→ ISSUE INTENT / GOVERNING AUTHORITY
→ EXPECTED BEHAVIOR
→ IMPLEMENTATION / ACTUAL RUNTIME PATH
→ TESTS / EVIDENCE
→ PLATFORM / INTEGRATION SCOPE
→ HANDOFF STATE
```

## Three correctness questions

Answer independently:

1. **Implementation correctness** — does the implementation satisfy the original issue and governing semantics?
2. **Verification correctness** — does the evidence actually establish the required properties, paths, platforms, negative cases, and regressions?
3. **Integration/handoff completeness** — is the correct candidate truthfully based, identified, linked, and reviewable against relevant current repository state?

Any one can independently require changes.

## VerbaTap review lenses

Apply only those material to the issue.

### Actual application path

Where user-visible behavior is involved, trace far enough to establish the real path, for example:

```text
frontend/UI
→ Tauri boundary
→ Rust responsibility
→ native/platform implementation
→ observable behavior
```

Do not approve based solely on a helper/unit test if production behavior bypasses or conditionally replaces it.

### Platform and architecture scope

Ask whether the implementation and evidence match the claim.

```text
TARGET COMPILES
!=
TARGET RUNTIME VERIFIED

ONE OS VERIFIED
!=
CROSS-PLATFORM VERIFIED
```

Inspect platform-gated code and affected target-specific dependencies/configuration where material.

### Local-first / privacy / network

When relevant, verify that default/local behavior remains local and that external processing/network transfer occurs only under the intended explicit conditions.

### Persistence and compatibility

Check current/legacy persisted values, environment variables, public configuration, migration paths, and compatibility identifiers where the change touches them.

Do not accept a cleanup/rename merely because an inherited identifier looks obsolete.

### Handy/upstream material

Classify inherited references/dependencies/behavior before treating them as defects or required preservation. Verify that changes do not erase needed provenance/compatibility or blindly preserve upstream product identity.

### Models and packages

Distinguish package/catalog/download readiness from actual inference/runtime support. Verify the layer the issue actually claims.

### Generated and packaged surfaces

Check whether generated bindings/artifacts were updated consistently when a boundary changed. For packaging/install changes, inspect the relevant build/artifact evidence rather than assuming source correctness implies package correctness.

## Review method

Trace every material required outcome to actual behavior/content/evidence.

Look for:

- omitted required outcomes;
- plausible but wrong interpretation;
- responsibility at the wrong layer;
- weak or implementation-shaped tests;
- invalid negative-test preconditions;
- wrong runtime/native path;
- missing compatibility/migration case;
- missing platform-specific branch;
- evidence on one platform presented as broader support;
- build/package success presented as runtime proof;
- legacy/fallback/bypass paths that remain reachable unexpectedly;
- under-rejection or over-rejection;
- forbidden partial state/side effects after failure;
- scope drift or unnecessary redesign.

For every material property ask:

> What plausible incorrect implementation could still pass the current tests/evidence?

Where practical, prefer discriminator evidence that distinguishes the intended behavior from a plausible wrong implementation.

Use `verifying-verbatap-change` to evaluate whether the evidence class matches the claimed property.

## Evidence classification

Use precise language:

- `INDEPENDENTLY VERIFIED` — reviewer directly established the property at the stated scope;
- `DIRECT SOURCE / DIFF INSPECTION` — established by source/artifact inspection rather than runtime;
- `INDEPENDENT HOSTED CI` — observed hosted verification, limited to the workflow/host/target/path actually exercised;
- `IMPLEMENTER-REPORTED / CORROBORATED` — implementer evidence agrees with independently inspected state;
- `IMPLEMENTER-REPORTED ONLY`;
- `INSUFFICIENT EVIDENCE`;
- `CONTRADICTED`.

Do not upgrade checked boxes, test names, configuration shape, mergeability, build artifacts, or external AI review status into stronger evidence than they provide.

## Materiality and stopping

Do not require a correction solely because stronger testing or broader platform coverage is imaginable.

A mandatory correction should normally correspond to a plausible material failure, contract violation, ownership defect, required missing outcome, false support/privacy/compatibility claim, or meaningful verification/integration risk.

Once the required behavior, verification scope, relevant platform/compatibility implications, authority ownership, and handoff are sufficiently established, stop rather than ratcheting into hypothetical hardening.

## Snapshot-bound approval

Approval applies to a specific candidate state. Record at least:

- reviewed PR head SHA;
- reviewed base/current-main SHA;
- material evidence limitations, including platform/target limitations where relevant.

If head or relevant base later changes, approval becomes stale until proportional reconciliation/re-review establishes the new state.

## Output

Report:

1. issue intent;
2. expected behavior / acceptance conditions;
3. implementation assessment;
4. test/evidence assessment;
5. platform/integration assessment where material;
6. findings ordered by materiality;
7. unresolved risks/evidence limits;
8. reviewed head/base;
9. status.

For findings use:

```text
REQUIREMENT / INTENT
→ OBSERVED BEHAVIOR
→ EVIDENCE
→ IMPACT
```

Use one status:

- `APPROVED FOR HUMAN REVIEW` — no material implementation, verification, integration, or handoff defect remains on the reviewed basis;
- `CHANGES REQUIRED` — a material defect, missing outcome, verification defect, or integration/handoff defect remains;
- `REVIEW BLOCKED` — reliable judgment requires missing evidence/authority/access or unresolved upstream semantics.

Approval is not permission to merge.

## Finding routing

### Implementation / semantic defect

Use a bounded fresh correction context against the same issue and existing PR. Preserve original scope/semantics, correct only verified findings, perform affected verification, re-read/reconcile the original issue, verify handoff, then stop.

### Verification defect

If behavior appears correct but required evidence is insufficient, correct the evidence/test gap without redesigning semantics. Re-review new evidence and implementation surfaces affected by obtaining it.

### Integration / handoff defect

If semantics remain valid but candidate/base/template/metadata/handoff state is wrong or stale, use bounded integration refresh or handoff-only correction.

### Upstream semantic defect

If correct resolution requires changing human intent, architecture, specification, decomposition, prerequisites, or another governing decision, use Controlled Re-Entry to the earliest owner.

A stale base, merge conflict, relocated import, weak test, or metadata mistake is not Controlled Re-Entry unless it exposes an upstream semantic problem.

## Independent re-review

Re-review **delta-first, not delta-only**:

```text
PRIOR FINDING
→ PREVIOUS REVIEWED HEAD
→ CORRECTION / RECONCILIATION DELTA
→ CURRENT HEAD
→ FINDING CLOSURE
→ REGRESSION / OVERCORRECTION
→ AFFECTED ORIGINAL REQUIREMENTS
→ CURRENT INTEGRATION STATE
```

Revalidate only properties the delta could materially affect, while ensuring the original finding is actually closed and the correction did not create a new platform, privacy, compatibility, or ownership defect.
