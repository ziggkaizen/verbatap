---
name: evaluating-workflow-use
description: Use when real VerbaTap development work has produced enough evidence to assess whether the workflow reduced mistakes and friction or introduced unnecessary process cost.
---

# Evaluating Workflow Use

## Core principle

Evaluate the **development process**, not product correctness. Learn from real work without per-step telemetry or allowing one unusual event to rewrite the workflow.

## Capture observations only when useful

Useful events include:

- issue split/restart after implementation launch;
- misunderstood requirements;
- acceptance reconciliation catching omissions;
- independent review finding defects despite green tests;
- platform claim broader than available evidence;
- target build success masking native runtime uncertainty;
- compatibility identifier changed without sufficient migration discovery;
- Handy/upstream reference incorrectly removed or unnecessarily preserved;
- privacy/network behavior insufficiently scoped;
- parallel-safe work becoming stale/conflicted after another merge;
- final merge freshness requiring reconciliation;
- implementation complete but PR/issue handoff incomplete;
- GitHub mutation producing incorrect resulting state;
- correction broader/narrower than necessary;
- Controlled Re-Entry required, avoided, or incorrectly invoked;
- fresh context missing durable information;
- template field repeatedly confusing agents;
- verification requirement repeatedly impossible or excessive;
- guardrail preventing/exposing a material mistake;
- ceremony repeatedly adding no observable safety value.

A lightweight observation may contain:

```text
Context
Observed
Affected workflow concern (if identifiable)
Impact
Possible learning / hypothesis
```

Nothing more is required.

## Useful classifications

Distinguish where helpful:

- semantic/implementation defect;
- verification/evidence defect;
- platform-scope defect;
- integration/freshness defect;
- handoff/external-state defect;
- compatibility/migration defect;
- privacy/network defect;
- upstream semantic defect / Controlled Re-Entry;
- workflow friction / unnecessary ceremony.

## Periodic evaluation

Examine actual usage across:

### Work decomposition and execution

- issue sizing and fresh-context completion;
- issue splits/restarts;
- scope drift;
- parallel-safety quality;
- cross-platform work-unit boundaries;
- branch reconciliation when parallel work rejoins.

### Semantic control

- Controlled Re-Entry frequency/causes;
- whether upstream defects reached the correct owner;
- whether bounded integration drift was over-escalated;
- recurring missing compatibility/platform/privacy semantics.

### Verification and review

- material independent-review findings;
- false confidence from green tests/weak evidence;
- browser/build/unit evidence overstated as native runtime evidence;
- missing target/platform evidence;
- verification repeatedly broader than necessary;
- whether corrections closed findings without overcorrection.

### Integration and handoff

- reviewed PRs becoming stale;
- freshness checks catching real problems;
- issue/PR checkbox/template mismatches;
- implementation-complete but handoff-incomplete events;
- candidate repository state diverging from live GitHub/CI state.

### Agent/human burden

- information loss across fresh contexts;
- unexpected human intervention;
- template sections that agents cannot truthfully populate;
- repeated requests for unavailable platform hardware/evidence;
- skills/rules repeatedly adding no value;
- guardrails repeatedly catching mistakes.

### Executor/model/tool changes

Account for material changes in model/executor/tool capability, repository maturity, and issue mix. Do not attribute outcome changes solely to workflow changes when these changed too.

Use simple counts only when already available from normal work. Do not build telemetry infrastructure merely to measure the workflow.

## Evaluate safety value against cost

For a workflow rule ask both:

```text
What material failure did this prevent or expose?
What execution/review cost did it create?
```

Do not optimize only for fewer steps or maximum control.

## Evaluate correction routing

Check whether defects were handled at the smallest correct level:

```text
IMPLEMENTATION / SEMANTIC DEFECT
→ bounded implementation correction

VERIFICATION / EVIDENCE DEFECT
→ evidence/test correction

INTEGRATION / FRESHNESS DRIFT
→ integration refresh

HANDOFF / EXTERNAL STATE DEFECT
→ handoff-only correction + read-back

UPSTREAM SEMANTIC DEFECT
→ Controlled Re-Entry
```

Repeated use of a heavier response than necessary is workflow friction. Repeated use of a lighter response that leaves correctness unresolved is under-control.

## Output

Recommend one of:

- `KEEP`
- `STRENGTHEN`
- `SIMPLIFY`
- `MERGE`
- `REMOVE`
- `INVESTIGATE`

Prefer:

```text
OBSERVED EVIDENCE
→ WORKFLOW INTERPRETATION
→ PROPOSED CHANGE / NO CHANGE
```

over broad redesign.

Workflow evaluation proposes changes; it does not silently rewrite skill semantics.

## Reconcile workflow layers

VerbaTap workflow behavior is represented in both ChatGPT Project Sources and repository-local skills.

When evaluation changes behavior represented in more than one layer, update each owner only where it owns the affected concern. Do not mechanically mirror prose or move repository-local mechanics into Project Sources.

## Current hypotheses to watch

The following controls should continue to earn their place through actual use:

- one substantial issue per fresh implementation context;
- issue as execution contract with active acceptance-criteria checkboxes;
- independent review;
- platform/evidence-scope discipline;
- bounded fresh correction against the same issue/PR;
- semantic parallel execution with lightweight join/reconciliation;
- final merge freshness as a lightweight PR-local concern;
- implementation completion distinct from verified PR handoff;
- GitHub/external mutation read-back where resulting state matters.
