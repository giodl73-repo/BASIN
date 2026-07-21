# BASIN

**Water 2.0 — multi-scale water supply and conveyance analysis.**

**Water adequacy is not reservoir volume. It is a promise from source to user.**

BASIN scores water-system elements across supply, demand, storage, conveyance,
drought resilience, reuse, leakage, and access. It keeps scale, jurisdiction,
rights, hydrologic basis, and demand basis attached to every conclusion.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

> BASIN is a research and conceptual-design project. It is not an engineering
> study, hydraulic model of record, water-rights determination, environmental
> review, or advocacy brief, and it claims no agency, basin-authority, tribal,
> or international-body endorsement.

## Why this matters

A system can have water on paper and still fail during drought, peak demand,
conveyance loss, contamination, or rights conflict. BASIN turns those failure
modes into explicit dimensions and service tiers before a project is proposed.

The transferable principle is: **capacity claims are meaningless without their
time horizon, demand basis, conveyance path, and rights context.**

## What is implemented

| Crate | Responsibility |
|---|---|
| `basin-network` | Water sources, storage, conveyance, demand, and scale contracts. |
| `basin-corpus` | Evidence-labelled water corpus parsing and validation. |
| `basin-score` | DIM-01..13 score artifacts. |
| `basin-tier` | Tier-SLA classification and shortfall reporting. |
| `basin-gap` | Scale-filtered gap analysis and null-result reporting. |
| `basin-cli` | CLI front door for corpus, score, tier-SLA, and gap commands. |

## Evidence status

The implementation baseline is complete and fixture-backed. The next public
milestone is the first cited end-to-end water-system analysis with reproducible
source manifests.

## Quick start

```powershell
cargo run -p basin-cli -- --help
cargo run -p basin-cli -- gap --help
cargo test --workspace
```

## Method

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

Every element carries a scale and basin or jurisdiction. Supply-reliability
claims retain their hydrologic and demand basis.

## Documentation

- [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md) — scope, product shape, and next work.
- [`docs/vtrace/`](docs/vtrace) — VTRACE requirements, architecture, trace, and verification.
- [`context/waves/`](context/waves) — repo-local execution history.
- [`.roles/ROLE.md`](.roles/ROLE.md) — adversarial review panel.

## License

MIT. See [`LICENSE`](LICENSE).
