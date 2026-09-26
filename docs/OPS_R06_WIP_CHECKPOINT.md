# Operations R06 database foundation WIP checkpoint

Project: Symphosium. Branch `ops/r06-database-wip`, based on the R05-accepted revision `23b3f7f1a14eca8ed27b3812d9db10437149fb98`. This isolated branch contains Mid1's R06 database iteration only; it is not merged with the R09 development branch or canonical main. Operations copied the eight scoped developer artifacts byte-identically from the unchanged Mid1 worktree (`mid1-r06-db`). Checksums are listed in `OPS_R06_WIP_SHA256.txt`.

## Change scope

- Enable SQLx's `macros` feature needed by `sqlx::migrate!`.
- Add explicit PostgreSQL migration/connection boundary, scoped ticket creation and dependency functions, reversible project-management schema migrations, and ignored disposable-DB integration tests.
- Use a case-insensitive organization/email expression index and add the composite ticket key needed by scoped foreign keys.
- Preserve the source behavior that database connection does not run migrations automatically.

## Operations verification

Host: macOS arm64. Rust/Cargo 1.97.1. Set `RUST_BIN` to the installed toolchain `bin` directory; its direct binaries were used because the default PATH does not expose Cargo/Clippy. All compilation used an external target directory to avoid writing generated output into Mid1's worktree:

```sh
export PATH="$RUST_BIN:$PATH"
export RUSTC="$RUST_BIN/rustc"
export CARGO_TARGET_DIR=/private/tmp/symphosium-r06-wip-verify

cargo fmt --manifest-path rust/Cargo.toml --all -- --check
cargo clippy --manifest-path rust/Cargo.toml --all-targets --all-features --locked --offline -- -D warnings
cargo test --manifest-path rust/Cargo.toml --locked --offline --all-targets
cargo test --manifest-path rust/Cargo.toml --locked --offline --release --all-targets
cargo build --manifest-path rust/Cargo.toml --release --locked --offline
```

Every command above passed. Debug and release test suites each reported 21 passed, zero failed, and two intentionally ignored PostgreSQL tests.

The two ignored PostgreSQL tests were run against a fresh PostgreSQL 16 database in a task-created local container. The image digest was `sha256:1a6ab3f5345eb6dbe04a1349529caabdb0ab09293a09590fad07b2246bfa4b54`; the container port was bound only to `127.0.0.1:55432`. Tests reset the same database, so the harness was deliberately serialized:

```sh
PM_TEST_DATABASE_URL=postgres://symphosium_r06@127.0.0.1:55432/symphosium_r06 \
  cargo test --manifest-path rust/Cargo.toml --locked --offline \
  --test postgres_r06 r06_postgres -- --ignored --test-threads=1
```

Result: two passed, zero failed. Coverage includes migration up/down/up, idempotent standard-state seeding, organization/project scope rejection, concurrent ticket-number allocation, and rejection of competing inverse dependency edges. The PostgreSQL image/runtime and its Cargo dependencies are free local resources. The disposable database container was stopped and removed after verification; recreate it with a local PostgreSQL 16 image before rerunning the command.

The earlier missing `webpki-roots 0.26.11` archive was resolved by a locked Cargo fetch with approved host network access; no lockfile changes were made after the tested R06 snapshot was captured.

## Status and next owner

This is a work-in-progress Operations checkpoint, **not R06 acceptance**. Operations reproduced the listed checks on this exact source snapshot. Senior Developer 1 technical review and QA Engineer 2 independent acceptance remain required before integration into the development branch. R06 does not enable builtin mode, run migrations at boot, or implement the R08 services/API/authentication. No release, tag, or canonical-main integration is authorized by this checkpoint.
