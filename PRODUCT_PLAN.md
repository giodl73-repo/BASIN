# BASIN Product Plan

## Thesis

Score water systems at a declared scale, identify measurable supply,
conveyance, resilience, and access gaps, and design Water 2.0 interventions only
where evidence supports them.

## Implemented product shape

- Six-crate Rust workspace covering network, corpus, score, tier, gap, and CLI.
- International, national, regional, and local scale contracts.
- DIM-01..13 scoring and tier-SLA shortfall artifacts.
- Tail-versus-systemic gap classification.
- Deterministic tests and machine-readable CLI outputs.

## Current evidence

The implementation and fixture baseline are complete. The first cited
end-to-end water-system run, source manifest, and findings report remain the
next publication milestone.

## Next public work

1. Select a bounded source-backed municipal or basin-scale corpus.
2. Publish reproducible supply, demand, storage, and conveyance inputs.
3. Run tier, gap, and sensitivity analysis with explicit hydrologic basis.
4. Review the first gap-targeted intervention through the full panel.

## Non-goals

- No hydraulic model of record, rights allocation, or environmental review.
- No forecast of what agencies or basin authorities will build.
- No uncited supply, demand, yield, reliability, equity, or cost claim.
- No aggregation across scales without an explicit comparison basis.

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p basin-cli -- --help
```
