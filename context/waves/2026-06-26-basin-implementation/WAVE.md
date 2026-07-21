# Wave: BASIN Implementation

## Goal

Build the BASIN pipeline from the accepted work packages (WP-001..006), one work
package per pulse, each compiling and testing green before the next starts.

## Thesis

The left side of the V is settled (`docs/vtrace/`). This wave is the implementation build:
turn accepted work packages into tested Rust crates, bottom-up, with scale threaded
through the corpus and gap layers, and every pulse running the WP verification
commands and recording evidence back into the VTRACE trace.

## Pulse table

| Pulse | Work Package | Status | Outcome |
|------:|--------------|--------|---------|
| 01 | WP-001 `basin-network` | done | Water kernel: identity, connectivity, supply helpers, typed hydrologic basis. |
| 02 | WP-002 `basin-corpus` | done | Corpus model + scale/basin tags + schema + sources + evidence labels. |
| 03 | WP-003 `basin-score` | done | Dimension scoring DIM-01..13 + rubric record. |
| 04 | WP-004 `basin-tier` | done | Tier T1–T4 + SLA conformance + tier-SLA gap. |
| 05 | WP-005 `basin-gap` | done | Gap analysis (scale-filtered) + null result. |
| 06 | WP-006 `basin-cli` | done | CLI orchestration (`--scale`) + reproducible artifacts. |

## Success criteria

- Each work package meets its exit criteria and verification commands.
- Workspace stays green (`cargo fmt --check`, `cargo clippy -D warnings`,
  `cargo test --workspace`) after every pulse.
- `proof check .` stays clean.
- VTRACE trace/verification rows updated as each WP closes.
