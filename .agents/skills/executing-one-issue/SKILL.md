---
name: executing-one-issue
description: Use when one ready VerbaTap development issue has been launched into a fresh implementation context and must be completed without drifting into surrounding work.
---

# Executing One Issue

## Implementation handoff

A human operator selects one ready issue and launches a fresh implementation context.

The launch goal should remain small. The **issue owns task semantics**; `AGENTS.md` and repository skills govern execution mechanics.

For repository-changing work, use `references/pull-request-handoff.md` after implementation verification.

## Preconditions

Revalidate only what materially matters:

- the issue is still agent-ready and contains sufficient intent/scope/acceptance criteria;
- prerequisites are satisfied;
- material assumptions remain current;
- the base/branch/worktree is appropriate;
- known concurrent work does not materially conflict.

If correct implementation requires inventing missing human intent, target architecture, semantics, scope, prerequisites, or responsibility, stop rather than completing the plan inside the implementation context.

## Execution loop

1. Read the **complete original issue**.
2. Read applicable `AGENTS.md`, current templates when GitHub handoff is involved, and issue-referenced durable sources.
3. Inspect the actual current implementation relevant to the issue.
4. Revalidate prerequisites, base/working context, material assumptions, and relevant concurrent repository state.
5. Identify the acceptance-criteria checklist and use it as active progress state.
6. Implement **only this issue**. Choose low-level mechanics only where the issue/governing semantics intentionally leave them open.
7. Use `verifying-verbatap-change` to select proportionate verification for the affected surfaces and claimed behavior.
8. As outcomes become established, tick the corresponding issue acceptance-criteria checkbox **only after** the evidence supports it. Do not pre-check criteria as a plan.
9. For negative/failure behavior, verify intended preconditions and intended path where material.
10. Re-read the **original issue** after implementation.
11. Reconcile every required outcome as `VERIFIED`, `FAILED`, `BLOCKED`, or `NOT_IMPLEMENTED`, and ensure checkbox state remains truthful.
12. If every required outcome is `VERIFIED` and repository changes exist, complete the pull-request handoff.
13. Read back resulting external/GitHub handoff state rather than assuming requested mutations succeeded correctly.
14. Report implementation status, verification basis, PR handoff state, and material evidence limitations/integration observations; then stop.

## Completion boundaries

```text
IMPLEMENTATION COMPLETE
=
every required issue outcome VERIFIED
```

For repository-changing work:

```text
HANDOFF COMPLETE
=
implementation complete
+
correct issue / PR identity
+
focused PR exists
+
actual PR/base state verified
+
issue linkage and required handoff fields verified
+
acceptance-criteria state truthful
+
material repository drift reconciled or surfaced
```

Pull-request creation transfers the candidate to independent review. It is not approval or permission to merge.

```text
HANDOFF COMPLETE
!=
READY FOR HUMAN MERGE
```

## Repository drift during execution

If the issue remains semantically correct and drift is bounded:

```text
refresh / reconcile current base
→ resolve mechanical integration
→ rerun affected verification
→ verify resulting PR handoff
```

Treat this as integration refresh, not Controlled Re-Entry.

If reconciliation requires changing upstream intent, semantics, architecture, scope, prerequisites, or responsibility, stop and route to the earliest owner.

A stale base, relocated import, merge conflict, template change, metadata mistake, or other bounded integration problem is not by itself Controlled Re-Entry.

## Platform/evidence discipline

Do not expand claims beyond the verification actually obtained.

Examples:

```text
RUST TESTS PASS ON UBUNTU
!=
WINDOWS / MACOS NATIVE BEHAVIOR VERIFIED

PR BUILD ARTIFACT PRODUCED
!=
INSTALL / RUNTIME BEHAVIOR VERIFIED

PLAYWRIGHT GREEN
!=
TAURI DESKTOP INTEGRATION VERIFIED
```

Record material limitations instead of hiding them behind a broad completion statement.

## Parallel execution

Parallel-safe issues may diverge while executing:

```text
A + B execute independently
→ A merges
→ B re-evaluates its relationship to current main
→ reconcile only if required
```

Worktrees reduce working-copy collisions; they do not create semantic or integration safety.

## Stop boundary

After implementation and verified PR handoff: report and stop.

Do not merge, manually close the issue, select/start the next substantial issue, trigger a release, or independently review the implementation from the same context.
