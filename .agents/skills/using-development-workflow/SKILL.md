---
name: using-development-workflow
description: Use when repository-local VerbaTap work has multiple possible workflow concerns or it is unclear which installed development skill should be applied next.
---

# Using Development Workflow

Use the **earliest unresolved material concern** that could change downstream work. Skip concerns already sufficiently established for the current scope.

## Repository-local routing

| Question | Skill |
|---|---|
| Are material current-system facts unknown? | `discovering-current-reality` |
| Is one ready issue being implemented? | `executing-one-issue` |
| Is required verification/evidence unclear? | `verifying-verbatap-change` |
| Is one implementation being independently checked? | `reviewing-issue-implementation` |
| Is an already-reviewed PR still current and safe for final human merge? | `finalizing-reviewed-pr` |
| Has a human explicitly selected a merged state for release preparation? | `releasing-verbatap` |
| Has enough real usage accumulated to evaluate the workflow itself? | `evaluating-workflow-use` |

Pre-implementation concerns — framing, alignment, design, specification, decomposition, and writing the agent-ready issue — are owned by the VerbaTap ChatGPT Project Sources. Do not create repository-local substitutes for those concerns merely to keep work moving.

If execution discovers missing upstream intent, semantics, architecture, scope, prerequisites, or responsibility, stop affected work and route the missing concern back to its owner.

## Scale rigor with risk

Do not invoke a concern merely because it exists.

Examples:

- a documentation typo may require no implementation workflow ceremony;
- a focused frontend bug may need only current-reality inspection, one issue, relevant verification, review, and freshness;
- a platform-native, persistence, model/runtime, packaging, privacy/network, or compatibility change may require materially stronger discovery and evidence.

Repository or integration drift should normally be reconciled at the current boundary. Use Controlled Re-Entry only when continuation requires changing an upstream-owned semantic decision rather than merely refreshing integration state.

## Release routing

Release/publish work is intentionally outside the ordinary implementation path. A merge does not authorize release. Use `releasing-verbatap` only after explicit human selection of the release state, and preserve the human final-publish boundary.
