# Pulse 01: WP-001 `basin-network` water kernel

Status: pending. Executes WP-001 (see `docs/vtrace/WORK_PACKAGES.md`).

## Scope

The water graph kernel — the pipeline primitive every other crate depends on.
Implements the load-bearing identity and connectivity invariants and the typed
hydrologic basis (firm yield vs average) required by REQ-007.

## Planned changes

- `Cargo.toml` workspace (member `crates/basin-network`).
- `crates/basin-network/Cargo.toml` (deps: `petgraph`, `serde`, `thiserror`).
- `crates/basin-network/src/lib.rs`: `Node`, `Conveyance` (with typed `Basis`
  enum), `Network`, `NetworkError`; `add_node`/`add_conveyance` (identity +
  validation); `node_count`, `conveyance_count`, `degree`, `is_connected`,
  `incident_capacity_af`, connectivity proxy (DIM-04).

## Parent IDs

REQ-004/005/007 · SPEC-001/005 · IF-005 · PKG-001 · CR-001..008.

## Exit criteria

- Workspace compiles; `cargo test -p basin-network` green.
- Tests cover: build network; degree; connectivity vs gap; incident capacity;
  hydrologic basis preserved (firm/average); duplicate-node, non-positive
  capacity, unknown-node typed errors.
- No `unwrap`/`panic!` in lib paths except tests; `clippy -D warnings` clean.

## Validation

```powershell
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test -p basin-network
```

## VTRACE closeout (on completion)

VER-004/005/007 + EVID-CR-001..003 → passed; TRACE REQ-004/005/007 → implemented;
WORK_PACKAGES WP-001 → done; unblock WP-002.

## Status

Completed — the six-crate workspace and validation baseline are implemented.
