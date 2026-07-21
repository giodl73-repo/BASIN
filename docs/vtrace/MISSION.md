# Mission

## Scope

Repo: BASIN

VTRACE adoption scope: establish the mission baseline for BASIN before creating
requirements, specification baselines, trace rows, or work packages. This file is
the leftmost VTRACE artifact for the repo and anchors later `REQ-*`, `SPEC-*`,
`WP-*`, verification, and validation records. BASIN is greenfield: this mission
defines intent ahead of any implementation, and implementation must trace back to
the needs and constraints below.

## Mission Need

| ID | Need | Success Criteria | Status |
|---|---|---|---|
| NEED-001 | BASIN shall turn public water data (e.g. USGS, EPA, Reclamation, state agencies, UN/FAO AQUASTAT) into a reproducible scored corpus of existing water systems and elements. | A maintainer can regenerate the active corpus, score, and gap artifacts from documented commands, with source/proxy/heuristic labels preserved. | accepted |
| NEED-002 | BASIN shall identify and explain water gaps — supply-demand deficits, drought fragility, missing conveyance/transfer, leakage, under-used reuse, quality, and access inequity — without overstating the evidence. | Every material claim is tied to a data artifact, command, source label, confidence label, or review record. | accepted |
| NEED-003 | BASIN shall convert analysis into defensible conceptual Water 2.0 upgrade options, not engineering studies, water-rights determinations, or advocacy briefs. | Proposed projects and feature packages are labelled implemented, heuristic, simulated, planned, held, or deprecated, with the hydrologic and economic basis labelled before publication. | accepted |
| NEED-004 | BASIN shall keep system identity stable as analysis moves from raw sources/conveyance to scored systems, gap regions, and design proposals. | Element-bearing artifacts join through a stable source/conveyance/system identifier rather than a transient label, operator, or map id. | accepted |
| NEED-005 | BASIN shall expose water tradeoffs through adversarial review roles instead of hiding them behind a single score. | Parliament and editorial reviews can change claims, labels, next evidence steps, or promotion status. | accepted |
| NEED-006 | BASIN shall report a rigorous null result as a valid finding. | When the scored corpus shows a system is already resilient and equitable, the artifacts say so rather than manufacturing a gap. | accepted |
| NEED-007 | BASIN shall classify each system into a four-tier conveyance hierarchy (T1 Interbasin/Transboundary Transfer, T2 Regional Conveyance, T3 Local Distribution Trunk, T4 Service/Last-mile) and define supply-reliability, quality, continuity, and access SLAs per tier, so that "is water adequate here?" is answered against an explicit tier promise. | Every analyzed element carries a tier and a declared SLA, and adequacy claims are made against the tier SLA rather than an unstated baseline. | accepted |
| NEED-008 | BASIN shall apply the same methodology at multiple scales — international (transboundary basins and treaties), national, regional (interstate compacts, water districts), and local (municipal) — with every element tagged by scale and basin, and analysis runnable at a chosen scale. | Every corpus element declares a scale; scores, tiers, and gaps are interpreted within scale; a gap run can target a single scale without cross-scale leakage. | accepted |

## Users

| User | Need | Success Signal |
|---|---|---|
| BASIN maintainer | Know which commands, artifacts, and review gates define the current truthful repo state at a given scale. | A clean validation bundle runs and the resulting artifacts match the documented claims and declared scale. |
| Water-resources analyst | Inspect scored systems, gaps, and evidence labels without reverse-engineering the implementation. | Scores, gap maps, and reports cite their source surfaces, confidence posture, and scale. |
| Basin / utility planner | Understand why a system, tier, or project is supported, held, or downgraded. | Each claim names the data, scenario, role review, scale, and next evidence step that governs it. |
| Drought / resilience reviewer | See how BASIN handles firm yield, drought of record, and groundwater conceptually. | Supply-reliability claims expose their hydrologic basis and evidence level, not just an aggregate score. |
| Rights / transboundary stakeholder | See whether water rights, compacts, and treaties are represented honestly. | Allocation and transfer assumptions are explicit and bounded by rights, not assumed free. |
| Environmental / community reviewer | See ecological-flow and safe-access exposure before a project is promoted. | Flow and access claims point to data or held evidence, not narrative alone. |
| Coding agent | Make scoped changes without drifting claims, artifacts, scale, or review obligations. | Work packages name parent IDs, affected modules/data/docs, validation commands, and evidence rows before closure. |

## Operating Context

BASIN will be a data corpus, review system, and research/design process for Water
2.0, with an implementation built later from accepted VTRACE work packages. It is
**multi-scale by design**: the same corpus, dimension pool, and tier model apply
to a municipal system, a state, a shared river basin, or a transboundary treaty
region, and a run targets a stated scale. Work happens inside a dirty portfolio
checkout, so repo-local changes must stay scoped and must not depend on
TRACKER-relative paths for build correctness. BASIN is not yet a TRACKER submodule
until intake completes.

This mission file does not yet assert any scored result. It creates the VTRACE
anchor that later requirements, specifications, and work packages trace back to.

The tiering frame (NEED-007) is ROUTE/PYLON/GAUGE-shaped, and the scale frame
(NEED-008) extends the portfolio pattern: just as highways, grid, and rail are
tiered SLA systems, water is a tiered conveyance system — but uniquely one that
must be analyzed at whatever scale (local to international) the question demands.

## Constraints

| ID | Constraint | Rationale | Status |
|---|---|---|---|
| CON-001 | BASIN public claims must stay bounded by implemented commands, generated artifacts, source labels, confidence labels, and review records. | Prevents planned, heuristic, or simulated work from reading as proof-grade evidence. | accepted |
| CON-002 | Element-bearing artifacts must preserve stable source/conveyance/system identity; operators, project names, and map ids are not primary keys. | Keeps scores, gaps, and proposals tied to stable physical identity. | accepted |
| CON-003 | Generated artifacts must name the source-of-truth data and commands that regenerate them. | Keeps the repo reproducible and prevents hand-edited generated outputs from becoming hidden state. | accepted |
| CON-004 | Source gaps, heuristic rows, simulated evidence, and human/owner review holds must remain visible status, not missing prose. | Keeps evidence debt actionable and traceable. | accepted |
| CON-005 | BASIN implementation changes belong in this repo; TRACKER receives only intentional submodule pointer updates after intake. | Preserves portfolio snapshot discipline. | accepted |
| CON-006 | BASIN must not claim construction readiness, hydraulic/water-quality validity of record, water-rights determination, environmental clearance, or official agency/treaty endorsement. | Keeps the project framed as research, tooling, review, and conceptual design. | accepted |
| CON-007 | Every claim must declare its scale, and scores/tiers/gaps must not be compared or aggregated across scales without an explicit, labelled cross-scale note. | Prevents misleading mixing of local and basin/international evidence (NEED-008). | accepted |

## Non-Goals

- BASIN is not an engineering study, hydraulic model of record, or environmental
  review.
- BASIN is not a water-rights determination, allocation, or legal opinion.
- BASIN is not an advocacy brief for a specific project, utility, basin, or treaty.
- BASIN does not predict what agencies, basin authorities, states, or
  international bodies will build or allocate.
- BASIN does not treat illustrative maps or heuristic forecasts as proof-grade
  evidence unless their evidence level says so.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| VTRACE mission needs are explicit enough to derive requirements. | Inspect this file before writing `REQUIREMENTS.md`. | future `EVID-*` |
| Mission needs cover corpus reproducibility, evidence posture, design boundaries, identity, review roles, null-result discipline, tiered SLAs, and multi-scale applicability. | Cross-check against `README.md`, `PRODUCT_PLAN.md`, and `CLAUDE.md`. | future `EVID-*` |
| Later VTRACE artifacts can reference stable parent IDs. | `REQ-*` rows should cite `NEED-*` and `CON-*` IDs from this file. | future `TRACE.md` |

## Role Review Notes

| Role Lens | Mission Impact | Disposition |
|---|---|---|
| Scope Keeper | Mission stays at repo/system intent; it asserts no scores, gap findings, or design proposals, and names the multi-scale rule. | pass |
| Citation Auditor | Mission makes no quantitative claims; source links are repo-local context artifacts. | pass |
| Numeracy Checker | Mission contains no arithmetic, supply, demand, or cost claims. | pass |
| Basin Planner | Mission names basin-wide balance, tiering, multi-scale, and public-interest intent. | pass |
| Drought Hydrologist | Mission requires hydrologic-basis framing for supply reliability (NEED-002/003). | pass |
| Water-Rights Realist | Initial draft underplayed rights/compacts/treaties; resolved by adding the Rights/transboundary user lens and CON-006 rights-determination boundary. | resolved |
| Environmental-Flow & Equity advocates | Mission names ecological flow and safe-water access as first-class via users and NEED-002. | pass |

Fixed-point note: one actionable finding (water rights/transboundary governance
under-represented) was raised and applied. No unresolved critical or major finding
remains. Deferred: dimension pool, scoring rubric, tier SLA thresholds, hydrologic
methodology, and the scale-tagging schema to REQUIREMENTS and SPECIFICATION_BASELINE.

## Source Links

- `README.md`
- `PRODUCT_PLAN.md`
- `CLAUDE.md`
- `.roles/ROLE.md`
