---
name: finalizing-reviewed-pr
description: Use immediately before human merge when an independently reviewed VerbaTap pull request must be checked for review-snapshot freshness against the current repository and merge state.
---

# Finalizing Reviewed PR

## Core principle

Determine whether the independently reviewed PR is still the same reviewed candidate and sufficiently current against today's repository state.

This is a lightweight freshness/reconciliation concern. It is not a second full issue review, not a replacement for independent review, and not a global readiness stage.

## Preconditions

Use only after independent issue review has established an acceptable reviewed candidate.

Know or recover:

- reviewed PR head SHA;
- reviewed base/current-main SHA;
- current PR head SHA;
- current `main` SHA;
- current merge/integration checks relevant to this repository and change;
- material evidence limitations from the review.

## Freshness check

Verify:

```text
reviewed PR head == current PR head
reviewed/reconciled base == current main
PR integration state == current and conflict-free
relevant verification / CI evidence == fresh enough for current candidate
material issue / PR handoff state == truthful
```

Where hosted CI/build evidence matters, confirm it corresponds to the current head/base/merge-ref semantics actually used by that workflow.

`mergeable = true`, successful target compilation, or an old green workflow run does not establish review freshness by itself.

## If the snapshot is still current

If reviewed head/base remain current and relevant checks/evidence remain valid:

```text
REVIEW SNAPSHOT CURRENT
→ READY FOR HUMAN MERGE
```

Report head/base/evidence basis. Do not merge.

## If head or base changed

Inspect the actual delta and classify it:

- **no material interaction**;
- **bounded integration drift**;
- **semantic conflict**.

For non-semantic drift:

```text
reconcile candidate
→ inspect delta / integration interaction
→ rerun affected verification only
→ refresh relevant merge-state evidence
→ proportional delta re-review
```

For a pure rebase/reconciliation, use diff/blob equivalence or similar evidence where useful to establish that previously reviewed production/test semantics remain unchanged.

For semantic conflict, route the earliest owning concern through Controlled Re-Entry.

## Platform/build considerations

A base/head change may invalidate target/build evidence even when application semantics did not change, especially when dependencies, Tauri configuration, native build files, workflows, or target-specific code moved.

Re-run only the evidence classes the delta can materially affect, but do not keep stale cross-target/package evidence merely because the source diff looks small.

## Parallel join

Correctly parallel work still needs a join evaluation when one branch merges and changes the base for a surviving sibling:

```text
A + B execute independently
→ A merges
→ evaluate B against new current main
→ reconcile B only when required
→ preserve focused diff
→ proportional affected verification / re-review
→ refresh merge-state evidence
→ final freshness
→ human merge
```

This does not imply A and B should have been serialized.

## Result

Report:

- current PR head;
- current `main`;
- relationship to reviewed head/base;
- reconciliation performed, if any;
- current relevant verification/CI/merge-state evidence;
- material review evidence limitations still applicable;
- unresolved blockers, if any.

Use one result:

- `READY FOR HUMAN MERGE`;
- `RECONCILIATION REQUIRED`;
- `FINALIZATION BLOCKED`.

The human owns the final merge decision and action. Finalization does not authorize or trigger a release.
