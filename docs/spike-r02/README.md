# Symphosium R02 bounded spike

Run with Rust 1.97.1's toolchain bin directory on PATH:

```sh
cargo fmt --all -- --check
cargo clippy --offline --locked --all-targets -- -D warnings
cargo test --offline --locked
cargo run --offline --locked
cargo build --release --offline --locked
```

The provider-client test binds only `127.0.0.1`; the sandbox may deny socket binding, requiring the approved unsandboxed execution used in the evidence log. The ignored SQLx test requires a disposable PostgreSQL URL in `R02_PG_URL` and `cargo test postgres_ddl_and_transaction_rollback -- --ignored`. Never point it at a shared database. This package is a feasibility spike, not application code or runtime parity proof.
