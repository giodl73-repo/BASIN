# BASIN — Role Index

Four tiers of review roles. Read this before opening any role file. Reviews of
corpus entries, gap findings, design proposals, tier/SLA definitions, and VTRACE
deliverables run against these files and record dispositions
(`pass` / `finding` / `defer`).

---

## Parliament roles (7 voices)

Adversarial expert voices. They plant incompatible stakes; the argument record is
the output, not consensus. No voice is skipped. A good project survives all seven;
a weak one collapses under one or two, and the collapse is the finding.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/basin-planner.md` | Basin / Water-Resources Planner | Basin-wide balance vs. single-jurisdiction framing |
| `parliament/hydraulic-engineer.md` | Hydraulic / Civil Engineer | Buildable hydraulics vs. map-fantasy transfers |
| `parliament/drought-resilience-hydrologist.md` | Drought & Climate Resilience Hydrologist | Firm drought yield vs. average-year optimism |
| `parliament/water-economist.md` | Water Economist | Benefit-cost + conservation vs. new-supply default |
| `parliament/environmental-flow-advocate.md` | Environmental-Flow Advocate | Ecological flows vs. human diversion |
| `parliament/equity-access-advocate.md` | Equity & Safe-Water Access Advocate | Safe affordable access vs. supply-optimized bypass |
| `parliament/water-rights-realist.md` | Water-Rights & Transboundary Realist | Rights/compacts/treaties vs. free-transfer assumptions |

---

## Editorial roles (3 voices)

Form gate before `validated` status. Run after parliament, not instead of it.

| File | Role | Checks |
|---|---|---|
| `editorial/citation-auditor.md` | Citation Auditor | Every quantity sourced in `data/sources.md` or labelled |
| `editorial/scope-keeper.md` | Scope Keeper | Artifact stays within its declared type, **scale**, schema, pool, and tier model |
| `editorial/numeracy-checker.md` | Numeracy Checker | Units consistent (af/m³/MGD/mg·L⁻¹/$); magnitudes sane; arithmetic and 0–10 scale clean |

---

## Stakeholder roles (cross-cutting lenses)

Not reviewers — lenses for who the system serves, used during corpus scoring, gap
analysis, and tier/SLA assignment.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/municipal-utility.md` | Municipal Water Utility | Supply reliability, quality, leakage, rates |
| `stakeholders/agricultural-irrigator.md` | Agricultural Irrigator | Allocation reliability, timing, cost, rights |
| `stakeholders/tribal-water-rights.md` | Tribal / Indigenous Water-Right Holder | Reserved rights, safe access, cultural/ecological flows |
| `stakeholders/downstream-user.md` | Downstream / Riparian User | Flows reaching downstream, quality, transboundary fairness |
| `stakeholders/household-ratepayer.md` | Household Ratepayer | Safe affordable water, continuity, bill impact |

---

## Panel reviewer roles (illustrative peer panel)

Archetype academic/practitioner peer reviewers for BASIN research outputs. See
`panel-reviewer/panel.md`. Used for paper-grade methodology review, distinct from
parliament and editorial.

---

## How reviews are recorded

When a `docs/vtrace/` deliverable, corpus entry, gap finding, design proposal, or
tier/SLA definition is being settled, the relevant subset of this panel is applied
and dispositions are recorded in:

- the deliverable's **Role Review Notes** section, and
- the active wave pulse ledger.

A stage reaches its **fixed point** when no unresolved critical or major
actionable finding remains and every deferred item names a later stage or work
package.
