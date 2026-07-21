# Contributing

Keep BASIN scale-aware, evidence-labelled, and explicit about the difference
between analysis and engineering, rights allocation, or advocacy.

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p basin-cli -- --help
```

Do not commit raw restricted datasets, credentials, local build state, or
uncited public claims.
