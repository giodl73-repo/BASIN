# Specification Baseline

## Scope

Repo: BASIN

Baseline type: target (provisional)

Baseline date: 2026-06-26

VTRACE adoption scope: define the controlled behavior BASIN intends to build — the
dimension pool, scoring scale, hydrologic basis, corpus schema, evidence labels,
tier model, and the **multi-scale model** — before architecture, interfaces, or
implementation planning. Because BASIN is greenfield, every item is `target`, not
observed `current`. The dimension pool is **provisional**: dimensions and their
basis are controlled here, but per-dimension anchors and rubric weights calibrate
from the scored corpus (REQ-006) and are not asserted in this file. Future work
packages must cite `SPEC-*` / `DIM-*` IDs instead of making unanchored changes.

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| `README.md` | BASIN thesis, hypothesis, multi-scale, pipeline. | target | Public-facing repo intent. |
| `PRODUCT_PLAN.md` | Scope, non-goals, method, waves. | target | Product framing. |
| `CLAUDE.md` | House rules, multi-scale rule, pipeline, quality bar. | target | Operating constraints. |
| `docs/vtrace/MISSION.md` | `NEED-*`, `CON-*`. | current | VTRACE mission source. |
| `docs/vtrace/CONOPS.md` | `OPS-*` scenarios. | current | VTRACE scenario source. |
| `docs/vtrace/REQUIREMENTS.md` | `REQ-001..016`, `DEF-001..005`. | current | VTRACE requirement source. |
| `.roles/ROLE.md` | Parliament + editorial review lenses. | current | Review-lane source. |

## Scale Model (`SCALE-*`) (resolves NEED-008 / REQ-016)

BASIN runs the same methodology at any scale. Every corpus element declares a
scale; scores, tiers, and gaps are interpreted within scale.

| Scale | Meaning | Example governance |
|---|---|---|
| `international` | Transboundary basins shared by nations. | Treaties, international commissions. |
| `national` | A single nation's systems and policy. | National water agencies. |
| `regional` | Multi-jurisdiction within a nation. | Interstate compacts, water districts, basin authorities. |
| `local` | A municipal/utility/community system. | Utilities, irrigation districts. |

| ID | Rule |
|---|---|
| SCALE-01 | Every corpus element carries a `scale` and a `basin`/jurisdiction tag (REQ-016). |
| SCALE-02 | Scores, tiers, and gaps are interpreted within the element's scale; cross-scale comparison/aggregation requires an explicit labelled note (CON-007). |
| SCALE-03 | Scale may nest (a local system within a basin within a treaty region); nesting representation is provisional (DEF-005). |

## Dimension Pool (`DIM-*`)

The candidate pool BASIN scores existing water systems against. Each dimension is
scored 0–10. Anchors and weights are **calibrated from the corpus** (REQ-006), not
fixed here. "Primary basis" names where the input comes from; "Default label" is
the evidence posture a fresh value carries until upgraded with a cited source
(REQ-002, REQ-003).

| DIM ID | Dimension | What it measures | Primary basis | Default label |
|---|---|---|---|---|
| DIM-01 | Supply–Demand Balance | Available supply vs. demand at the stated scale. | USGS water use, state/UN data | source-needed |
| DIM-02 | Drought / Climate Resilience | Firm yield under the drought of record and a changing climate. | Hydrologic records + climate projections | heuristic |
| DIM-03 | Storage Adequacy | Reservoir + aquifer carryover storage through multi-year drought. | Reclamation / USGS storage data | source-needed |
| DIM-04 | Conveyance / Transfer Capacity | Hydraulic capacity to move water where needed (graph). | Network topology + capacity | heuristic |
| DIM-05 | Water Quality / Treatment | Delivered-water safety and treatment adequacy. | EPA SDWIS / WHO | source-needed |
| DIM-06 | Leakage / Non-Revenue Water | Volume lost between source and use. | Utility water audits | proxy |
| DIM-07 | Reuse / Recycling Potential | Untapped potential for reuse/recycling as a source. | Effluent + reuse studies | heuristic |
| DIM-08 | Ecological / Environmental Flow | In-stream, wetland, and estuary flow left after diversion. | Environmental-flow studies | heuristic |
| DIM-09 | Groundwater Sustainability | Aquifer balance and depletion risk. | USGS / state groundwater data | source-needed |
| DIM-10 | Energy Intensity | Energy to lift, convey, and treat the water. | Operator / energy data | proxy |
| DIM-11 | Equity / Safe-Water Access | Who has safe, affordable, reliable water vs. who does not. | Census, SDWIS, EJ (computable) | implemented |
| DIM-12 | Capital-Efficiency / Benefit-Cost | Benefit per unit capital. | Planning B/C studies | heuristic |
| DIM-13 | Tier-SLA Conformance | Degree the element meets its assigned tier's reliability/quality/continuity/access SLA (derived; shortfall = tier-SLA gap). | Tier model + DIM-01/02/05/11 | heuristic |

Calibration note (per REQ-006, OPS-002): after the first corpus pass, low-variance
or redundant dimensions are retired and informative ones promoted; the rubric
version records each change. The pool above is the v0 candidate set, not a final
rubric.

## Hydrologic Basis (resolves DEF-002 minimum)

| ID | Rule |
|---|---|
| SPEC-HB-01 | Supply-reliability and yield dimensions (DIM-01, DIM-02, DIM-03) name the **hydrologic basis** — firm yield under the drought of record vs average-year — on every derived claim (REQ-007). |
| SPEC-HB-02 | Explicit multi-year drought-sequence modelling may be a labelled proxy when full records are unavailable; the proxy status must be explicit. Default scope names the basis (DEF-002 remains open for explicit sequence modelling). |

## System Tier Model (`T1–T4`) (resolves NEED-007 / REQ-014 / REQ-015)

BASIN classifies each system into a four-tier conveyance hierarchy — spanning
interbasin transfers to last-mile delivery — with a supply/quality/continuity/access
SLA per tier. This is the Water 2.0 analog of ROUTE/PYLON/GAUGE tiering. The tier
hierarchy nests within the scale model (a T1 transfer may itself be
international-scale). Roles are typical, not strict.

| Tier | Name | Typical role | SLA promise (target) |
|---|---|---|---|
| T1 | Interbasin / Transboundary Transfer | Major aqueducts/canals moving water across basins or borders. | Firm interbasin delivery; quality at transfer; compact/treaty-compliance posture. |
| T2 | Regional Conveyance | Regional trunk mains, source→treatment→distribution backbone. | Regional supply reliability; treated-quality; continuity. |
| T3 | Local Distribution Trunk | Municipal supply mains. | Pressure/continuity; quality; reliability under drought. |
| T4 | Service / Last-mile | Delivery to users/parcels. | Continuous, safe, affordable delivery; access. |

Each tier's SLA is expressed over four contract terms, assessed by DIM-13:

| SLA term | Meaning | Backing dimensions |
|---|---|---|
| Supply reliability | Firm delivery the tier promises, with hydrologic basis named. | DIM-01, DIM-02, DIM-03 |
| Water quality | Treated-water safety the tier must meet. | DIM-05 |
| Continuity | Pressure/continuity of service. | DIM-04, DIM-06 |
| Access / affordability | Who the tier reaches and at what affordability. | DIM-11 |

SLA values per tier are **target and provisional** — exact thresholds calibrate
with the rubric (REQ-006) and are not asserted here. A tier-SLA shortfall is a
first-class gap (REQ-015, OPS-006).

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | C/T/D/U | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-004 / REQ-005 | architecture | target | Every element is keyed by a stable source/conveyance/system identifier; operator, project name, and map id are mutable presentation fields, not keys. | schema check / inspection | OPS-001 | BASIN maintainer | high | accepted |
| SPEC-002 | REQ-001 / REQ-003 / REQ-014 / REQ-016 | product | target | A corpus entry is one markdown file with frontmatter (id, type, scale, basin, termini, tier, sla, source rows) and a scored dimension block, regenerable from documented commands. | inspection / command review | OPS-001 | BASIN maintainer | medium | accepted |
| SPEC-003 | REQ-002 | product | target | Every quantity carries an evidence label from {implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited}. | artifact inspection | OPS-001 / OPS-004 | BASIN maintainer | medium | accepted |
| SPEC-004 | REQ-006 | product | target | The dimension pool is `DIM-01..DIM-13` scored 0–10; anchors and weights are calibrated from corpus variance and versioned, not fixed in this baseline. | calibration record / version diff | OPS-002 | BASIN maintainer | high | accepted |
| SPEC-005 | REQ-007 | software | target | Supply/yield dimensions name the hydrologic basis (firm yield vs average) on each claim (SPEC-HB-01). | analysis / inspection | OPS-003 | drought hydrologist | high | accepted |
| SPEC-006 | REQ-008 | product | target | An already-resilient, equitable system is recorded as a labelled null result; scope is not expanded to manufacture a gap. | gap-artifact inspection / review | OPS-003 | BASIN maintainer | medium | accepted |
| SPEC-007 | REQ-009 / REQ-010 | ops | target | Promotable claims pass the 7-voice parliament and 3-role editorial gate, with supply, drought, conveyance, quality, ecological-flow, equity, benefit-cost, and water-rights lenses represented. | review inspection | OPS-004 | review steward | medium | accepted |
| SPEC-008 | REQ-011 | product | target | Outputs carry a scope boundary: research/tooling/conceptual-design only; no construction readiness, hydraulic/quality validity of record, water-rights determination, or endorsement. | editorial review | OPS-004 | BASIN maintainer | medium | accepted |
| SPEC-009 | REQ-003 | data | target | `data/sources.md` is the citation registry; every cited quantity names a registry entry, and proxies/heuristics are labelled rather than cited. | citation audit | OPS-001 | data steward | high | accepted |
| SPEC-010 | REQ-012 / REQ-013 | ops | target | VTRACE deliverables advance one at a time to a `.roles` fixed point; BASIN changes stay in the child repo until an intentional TRACKER pointer update after intake. | wave ledger / git status | OPS-005 | BASIN maintainer | low | accepted |
| SPEC-011 | REQ-014 | product | target | Every analyzed element is classified into exactly one tier (T1–T4) per the System Tier Model and carries that tier's declared SLA terms. | schema check / inspection | OPS-006 | BASIN maintainer | high | accepted |
| SPEC-012 | REQ-015 | software | target | Tier-SLA conformance (DIM-13) is assessed per element against its tier SLA; any shortfall is recorded as a tier-SLA gap and a system is not called adequate while an unaddressed shortfall stands. | analysis / gate / inspection | OPS-003 / OPS-006 | BASIN maintainer | high | accepted |
| SPEC-013 | REQ-016 | product | target | Every element carries a `scale` and `basin`/jurisdiction tag (SCALE-01); analysis runs within a scale and any cross-scale comparison carries an explicit labelled note (SCALE-02). | schema check / gate / review | OPS-007 | BASIN maintainer | high | accepted |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 / SPEC-002 / SPEC-013 | corpus file (markdown + frontmatter, incl. scale/basin) | Frontmatter keys are additive; `id` immutable; `scale` from a fixed enum. | Any key rename/removal, id-semantics, or scale-enum change. | schema check (target) |
| IF-002 | SPEC-009 | `data/sources.md` (registry) | Source entries are append/annotate; ids stable. | Removing or re-pointing a source id. | citation audit (target) |
| IF-003 | SPEC-004 | rubric version record | Dimension set + weights versioned; changes recorded. | Retiring/adding a `DIM-*` or changing weights. | calibration record (target) |
| IF-004 | SPEC-011 / SPEC-012 | tier/SLA record | Tier set (T1–T4) and per-tier SLA terms are versioned; tier reassignment is recorded. | Changing a tier definition, SLA term, or an element's tier. | tier/SLA record (target) |

## Package / Language Allocation

| Spec IDs | Package / Module | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| SPEC-001 / SPEC-004 / SPEC-005 | water kernel (future `basin-network`) | Graph model, conveyance metrics (DIM-04), supply/yield helpers. | Scoring policy, evidence labels, review logic. | L1 |
| SPEC-002 / SPEC-003 / SPEC-009 / SPEC-013 | corpus + data layer | File schema, scale/basin tags, source registry, evidence labels. | Graph math, design proposals. | L0/L1 |
| SPEC-007 / SPEC-008 | review layer (`.roles`) | Parliament/editorial gate, scope boundary. | Computing scores. | L0 |
| SPEC-011 / SPEC-012 | tier/SLA layer | Tier classification, SLA terms, tier-SLA conformance (DIM-13). | Setting calibrated SLA thresholds without rubric. | L1 |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-NF-001 | SPEC-002 / SPEC-004 | Reproducibility | Active corpus/score/gap artifacts regenerate from documented commands with labels and scale preserved. | command review | proposed |
| SPEC-NF-002 | SPEC-009 | No raw datasets committed | Raw/cache data is gitignored; only derived, cited artifacts are committed. | inspection | proposed |
| SPEC-NF-003 | SPEC-001 / SPEC-013 | Deterministic identity + scale | Element ids and scale tags are deterministic given source inputs. | inspection / test | proposed |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| SPEC-UNK-001 | Supply (DIM-01), quality (DIM-05), groundwater (DIM-09) depend on data of varying openness across scales. | May force proxy/source-needed labels on early corpus rows. | discovery → `data/sources.md` | data steward |
| SPEC-UNK-002 | Firm-yield/drought-of-record records (DIM-02) vary by basin and nation. | Likely proxy at v0 for data-poor basins. | accept risk (labelled proxy) | drought hydrologist |
| SPEC-UNK-003 | Benefit-cost (DIM-12) requires study assumptions. | Heuristic until grounded. | defer to corpus calibration | water economist |
| SPEC-UNK-004 | Per-tier SLA thresholds (DIM-13). | Affects conformance scoring. | defer to calibration | BASIN maintainer |
| SPEC-UNK-005 | Whether scale nests as a hierarchy or stays a flat tag. | Affects schema + cross-scale notes. | defer (DEF-005) | BASIN maintainer |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-002, SPEC-NF-001 | covered | Regeneration path. |
| REQ-002 | SPEC-003 | covered | Evidence labels. |
| REQ-003 | SPEC-009 | covered | Citation registry. |
| REQ-004 | SPEC-001 | covered | Stable identity. |
| REQ-005 | SPEC-001, SPEC-013 | covered | Hold/reject unidentified/untagged rows. |
| REQ-006 | SPEC-004, IF-003 | covered | Calibrated rubric. |
| REQ-007 | SPEC-005, SPEC-HB-01 | covered | Hydrologic basis named. |
| REQ-008 | SPEC-006 | covered | Null result. |
| REQ-009 | SPEC-007 | covered | Review gate. |
| REQ-010 | SPEC-007 | covered | Stakeholder lenses. |
| REQ-011 | SPEC-008 | covered | Scope boundary. |
| REQ-012 | SPEC-010 | covered | Child-repo scoping. |
| REQ-013 | SPEC-010 | covered | One-at-a-time VTRACE. |
| REQ-014 | SPEC-011, IF-004 | covered | Tier classification + SLA. |
| REQ-015 | SPEC-012, DIM-13 | covered | Tier-SLA gap gating. |
| REQ-016 | SPEC-013, SCALE-01..03, IF-001 | covered | Multi-scale tagging + within-scale interpretation. |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| SPEC-001..013 | future `VER-*` in `VERIFICATION.md` | Each spec has a credible check (schema, command, inspection, or review). | future `EVID-*` | planned |

## Role Review Notes

| Role Lens | Spec Impact | Disposition |
|---|---|---|
| Scope Keeper | Baseline defines controlled behavior, a candidate pool, a tier model, and the scale model; it asserts no scored system or design. | pass |
| Citation Auditor | No quantities asserted; "Primary basis" names where inputs come from; DIM default labels enforce citation discipline. | pass |
| Numeracy Checker | Only the 0–10 scale is named; the system `scale` enum is distinct from the score scale; no computed values. | pass |
| Drought Hydrologist | Hydrologic basis is controlled (SPEC-HB-01 / SPEC-005); explicit sequence modelling deferred. | pass |
| Water-Rights Realist | Initial draft let DIM-01 read as if supply were free of rights; resolved by adding the compact/treaty-compliance term to the T1 SLA and keeping rights in SPEC-007. | resolved |
| Water Economist | Benefit-cost (DIM-12) default label set to `heuristic`; SPEC-UNK-003 records the gap. | pass |
| Environmental-Flow & Equity advocates | Ecological flow (DIM-08) and equity access (DIM-11) are in the pool. | pass |

Fixed-point note: one actionable finding (supply read as rights-free) was raised
and applied via the T1 compact/treaty term + SPEC-007. No unresolved critical or
major finding remains. Pool, SLA, and scale-nesting details are explicitly
provisional; calibration and DEF-005 deferred.

## Specification Gate

Decision: pass_with_risk

Required before implementation planning:

- [x] Every accepted `REQ-*` maps to one or more `SPEC-*` IDs or a recorded deferral.
- [x] Every implementation work package can name parent `SPEC-*` IDs or discovery status.
- [x] Public contracts have owners and change-control triggers.
- [~] Unknowns are resolved, blocked, deferred, or converted to discovery work (SPEC-UNK-001..005 are discovery/defer/accept-risk).
- [x] Verification and validation methods are credible for the controlled claim.

Rationale: the baseline is coherent enough to drive trace, verification, and the
review gate. Residual risk is concentrated in cross-scale data openness
(SPEC-UNK-001/002), provisional weights/SLA thresholds, and scale-nesting
representation (DEF-005), all deferred to the corpus calibration wave rather than
blocking the minimum slice.
