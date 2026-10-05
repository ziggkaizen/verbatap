# VerbaTap Pull Request Handoff

## Purpose

A completed repository-changing issue is handed from implementation to independent review through one focused pull request.

The issue owns task semantics. The pull request owns the reviewable implementation candidate and handoff evidence.

```text
ONE ISSUE
→ ONE FRESH IMPLEMENTATION CONTEXT
→ ONE FOCUSED PR
→ INDEPENDENT REVIEW
→ FINAL MERGE FRESHNESS
→ HUMAN MERGE
```

Do not start the next substantial issue from the same implementation context.

## When to create the pull request

Create/update the PR only after:

1. issue implementation is complete;
2. required verification has been performed using the applicable VerbaTap verification model;
3. the original issue has been re-read;
4. every required outcome is `VERIFIED`;
5. acceptance-criteria checkboxes truthfully reflect that state.

If a required outcome is `FAILED`, `BLOCKED`, `NOT_IMPLEMENTED`, or materially `NOT VERIFIED`, do not represent the issue as complete.

If the issue legitimately produces no repository change, no PR is required.

## Branch and commit

Keep implementation isolated from `main`.

Before handoff:

- ensure intended issue changes are committed;
- exclude unrelated work;
- push the implementation branch;
- target the intended base (normally the default branch unless the issue explicitly says otherwise).

Do not expand scope merely to make the branch or PR look complete.

## Pull request content

Read and follow the **current** repository PR template.

The PR must make the handoff reviewable, including as applicable:

- linked implementation issue (`Closes #<issue-number>` when merge should close it);
- concise implemented outcome/change summary;
- material affected surfaces;
- verification actually performed;
- platforms/architectures/targets actually evidenced where material;
- material evidence limitations or `NOT VERIFIED` / `NOT AVAILABLE` properties;
- compatibility/migration impact when relevant;
- privacy/network impact when relevant;
- user-visible evidence when relevant.

Do not manually close the implementation issue.

Do not fabricate human testimony. If a currently installed template contains a truly human-only field, leave it for the human and report the incomplete field.

## Verify the handoff

Do not infer correct handoff from successful PR creation/update.

Read back actual GitHub state and verify:

- PR identity matches the intended issue/work unit;
- PR exists and targets the intended base;
- head/base relationship is current enough for truthful handoff, or material drift is surfaced;
- the diff is focused on the issue;
- the body links the correct issue;
- required template/checklist state is truthful;
- issue acceptance-criteria state remains truthful;
- issue remains open until merge when using merge linkage;
- any material GitHub metadata required by the current repository workflow is correct.

Correct wrong/missing handoff state before reporting completion.

## Drift before handoff

If repository state changed materially during implementation, reconcile affected assumptions before handoff.

Bounded mechanical drift is integration refresh. Upstream semantic drift requires Controlled Re-Entry.

A verified handoff is a truthful reviewable candidate, not final merge readiness. `finalizing-reviewed-pr` re-establishes freshness against current `main` after independent review.

## Stop boundary

After verified handoff:

1. report the PR URL/identity;
2. summarize implementation and verification evidence;
3. state material evidence limitations or integration observations;
4. stop.

Do not merge, manually close the issue, start the next substantial issue, perform independent review from the implementation context, or trigger/publish a release.
