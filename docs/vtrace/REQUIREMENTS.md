# Requirements

## Scope

Repo: BASIN

VTRACE adoption scope: derive initial repo-level requirements from
`docs/vtrace/MISSION.md` and `docs/vtrace/CONOPS.md`. These requirements describe
what BASIN must satisfy as analysis and implementation proceed; they do not by
themselves authorize implementation work — that comes from accepted work packages.
Requirements stay at contract level and assert no scores or designs.

## Requirement Table

| ID | Requirement | Parent Need / Constraint / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | BASIN shall maintain a documented regeneration path for the active corpus, score, and gap artifacts from public source data. | NEED-001 / CON-003 / OPS-001 | Reproducibility is the minimum condition for trusting generated claims. | must | BASIN maintainer | inspection / command review | accepted |
| REQ-002 | BASIN shall label every material quantity with an evidence posture (implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited). | NEED-002 / NEED-003 / CON-001 / CON-004 / OPS-001 / OPS-004 | Evidence labels prevent proxy or planned work from reading as proof. | must | BASIN maintainer | artifact inspection / review | accepted |
| REQ-003 | BASIN shall cite a declared source in `data/sources.md` for every quantity in a corpus entry, or mark it as a labelled proxy/heuristic. | NEED-001 / CON-003 / CON-004 / OPS-001 | Uncited numbers cannot be audited or regenerated. | must | data steward | citation audit / inspection | accepted |
| REQ-004 | BASIN shall identify each element by a stable source/conveyance/system identifier, not by a transient label, operator, or map id. | NEED-004 / CON-002 / OPS-001 | Stable physical identity is required before scores, gaps, and proposals can be compared. | must | BASIN maintainer | schema check / inspection | accepted |
| REQ-005 | BASIN shall hold or reject any corpus or gap artifact that lacks a stable element identifier, a declared source label, or a declared scale. | NEED-004 / NEED-008 / CON-002 / CON-004 / CON-007 / OPS-001 | Mutable labels, uncited rows, and untagged scale cannot safely join across analysis stages. | must | BASIN maintainer | gate / data inspection | accepted |
| REQ-006 | BASIN shall calibrate its scoring rubric from observed corpus variance and correlation, and record the rubric version and rationale for each change. | NEED-002 / NEED-005 / OPS-002 | Calibration must be evidence-driven and auditable, not asserted. | must | BASIN maintainer | calibration record / version diff | accepted |
| REQ-007 | BASIN shall ground supply-reliability and yield claims in an explicit hydrologic basis (firm yield under drought of record vs average year) and name the basis on the claim. | NEED-002 / CON-001 / OPS-003 / OPS-006 | Supply reliability is meaningless without stating the hydrologic basis. | must | drought hydrologist | inspection / review | accepted |
| REQ-008 | BASIN shall record a system that is already resilient and equitable as a valid null result rather than manufacturing a gap. | NEED-006 / CON-001 / OPS-003 | Silent scope expansion to rescue a hypothesis is forbidden. | must | BASIN maintainer | gap-artifact inspection / review | accepted |
| REQ-009 | BASIN shall route every promotable system or project claim through the 7-voice parliament and the 3-role editorial gate before downstream use. | NEED-005 / CON-001 / OPS-004 | BASIN's review system is part of the evidence model, not decoration. | must | review steward | review inspection | accepted |
| REQ-010 | BASIN shall represent supply, drought resilience, conveyance, quality, ecological flow, equity/access, benefit-cost, and water-rights posture in reviews or claim labels before a design option is promoted. | NEED-003 / NEED-005 / OPS-004 | These stakeholder lenses must remain first-class, per the mission users. | should | review steward | role review / inspection | accepted |
| REQ-011 | BASIN shall keep its outputs framed as research, tooling, review, and conceptual design — not construction readiness, hydraulic/quality validity of record, water-rights determination, or agency/treaty endorsement. | NEED-003 / CON-006 / OPS-004 | Scope control protects BASIN from overclaiming public authority. | must | BASIN maintainer | editorial review | accepted |
| REQ-012 | BASIN shall keep implementation and VTRACE changes scoped to the BASIN child repo until an intentional TRACKER submodule pointer update after intake. | CON-005 / OPS-005 | TRACKER is the snapshot repo; BASIN owns implementation history. | must | BASIN / portfolio maintainer | git status / submodule diff | accepted |
| REQ-013 | BASIN shall advance VTRACE deliverables one at a time to a `.roles` review fixed point, recording dispositions and deferrals. | NEED-005 / OPS-005 | The one-at-a-time discipline keeps each artifact reviewable and traceable. | must | BASIN maintainer | wave ledger / review notes | accepted |
| REQ-014 | BASIN shall classify every analyzed element into exactly one tier (T1 Interbasin/Transboundary Transfer, T2 Regional Conveyance, T3 Local Distribution Trunk, T4 Service/Last-mile) and attach the tier's declared SLA (supply reliability, quality, continuity, access). | NEED-007 / CON-002 / OPS-006 | A tiered SLA system requires every element to carry a tier and a promise it is judged against. | must | BASIN maintainer | schema check / inspection | accepted |
| REQ-015 | BASIN shall assess each element against its tier SLA and report any tier-SLA shortfall as a gap before a system is described as adequate. | NEED-007 / NEED-002 / NEED-006 / OPS-003 / OPS-006 | Adequacy must be measured against an explicit tier promise, not an unstated baseline; SLA gaps are first-class findings. | must | BASIN maintainer | gate / gap-artifact inspection | accepted |
| REQ-016 | BASIN shall tag every element with a scale (international/national/regional/local) and basin/jurisdiction, interpret scores/tiers/gaps within scale, and require an explicit labelled note for any cross-scale comparison or aggregation. | NEED-008 / CON-007 / OPS-007 | The multi-scale methodology is only sound if scale is explicit and not silently mixed. | must | BASIN maintainer | schema check / gate / review | accepted |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to a mission need, constraint, or CONOPS scenario.
- [x] Each requirement avoids implementation detail unless the detail is itself required.

## Role Review Notes

| Role Lens | Requirement Impact | Disposition |
|---|---|---|
| Scope Keeper | Requirements stay at contract level; REQ-016 makes scale a hard requirement. | pass |
| Citation Auditor | Requirements introduce no new numeric claims; REQ-003 hardens citation discipline. | pass |
| Numeracy Checker | No calculations, units, scores, supply, or cost claims. | pass |
| Basin Planner | Connectivity, tiering, and multi-scale intent preserved via REQ-014/016/010. | pass |
| Drought Hydrologist | Initial draft left the hydrologic basis implicit; resolved by adding REQ-007 (firm yield under drought of record named on the claim). | resolved |
| Water-Rights Realist | Water-rights posture required before promotion (REQ-010). | pass |
| Environmental-Flow & Equity advocates | Ecological flow and access required before promotion (REQ-010). | pass |

Fixed-point note: one actionable finding (hydrologic basis implicit) was raised and
applied as REQ-007. No unresolved critical or major finding remains.

## CONOPS Trace Review

| Scenario ID | Requirements Derived |
|---|---|
| OPS-001 | REQ-001, REQ-002, REQ-003, REQ-004, REQ-005 |
| OPS-002 | REQ-006 |
| OPS-003 | REQ-007, REQ-008 |
| OPS-004 | REQ-002, REQ-009, REQ-010, REQ-011 |
| OPS-005 | REQ-012, REQ-013 |
| OPS-006 | REQ-014, REQ-015 |
| OPS-007 | REQ-005, REQ-016 |

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| DEF-001 | Exact dimension pool and per-dimension definitions. | `SPECIFICATION_BASELINE.md` and first corpus-calibration wave. |
| DEF-002 | Whether supply scoring models multi-year drought sequences explicitly vs. as a proxy. | `SPECIFICATION_BASELINE.md` once the method is chosen. |
| DEF-003 | Specific data-source acquisition commands and refresh cadence. | `data/sources.md` and `VERIFICATION.md`. |
| DEF-004 | Implementation interfaces (CLI, schemas, crates). | `ARCHITECTURE.md` / `INTERFACES.md` after the minimum slice. |
| DEF-005 | Whether scale is a flat tag or a nested hierarchy (e.g. a local system within a basin within a treaty region). | `SPECIFICATION_BASELINE.md` / `INTERFACES.md`. |
