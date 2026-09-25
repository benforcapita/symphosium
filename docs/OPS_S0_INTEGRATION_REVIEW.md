# Operations S0 candidate integration review

Project: Symphosium, S0/R01 and R02. Canonical-linked repository branch: `ops/symphosium-s0`. Integration base: `cadcab18259adf7fdf541fa543d874f4d9bf5c2b`. Design authority remains `docs/PROJECT_LLD.md` revision 3, sections 11–12. This is an integration checkpoint for independent QA review, not a QA verdict or stage acceptance.

## Imported candidate

- PM handoff preserved byte-for-byte at `docs/S0_REVIEW_CANDIDATE.md`.
- Senior1 full R02 bounded spike patch integrated into `docs/spike-r02/` with updated source, 269-package lock, toolchain pin, Darwin arm64 license inventory/review and target dependency tree. `target/` build artifacts are ignored and excluded.
- Senior1 evidence preserved at `docs/R02_FULL_SPIKE_RESULT_v4.md`.
- Mid1 provider map at `docs/R01_PROVIDER_STAGE0_MAP.md` and Senior2 runtime map at `docs/R01_RUNTIME_STAGE0_MAP.md`.
- Existing QA2 review/ledger and fixture corpora remain intact. HLD, LLD3 and all 11 existing fixture corpus/manifest/checker files hash identically to the integration base. SHA-256 inventory is in `docs/OPS_S0_IMPORT_SHA256.txt`.

## Integrated local checks

Rust toolchain used: Rust/Cargo 1.97.1, Darwin arm64. The shell PATH does not include rustup's toolchain bin, so prepend:

```sh
export PATH="/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH"
cd docs/spike-r02
```

Commands run against the integrated contents:

| Command | Operations result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed. |
| `cargo clippy --offline --locked --all-targets -- -D warnings` | Passed. |
| `cargo test --offline --locked` in restricted sandbox | 4 passed; loopback provider test failed at `TcpListener::bind` with `Operation not permitted`; disposable-Postgres test ignored. |
| `cargo test --offline --locked` with approved normal host permissions | Passed: 5 passed, 0 failed, 1 ignored. Only the provider-client test binds `127.0.0.1`; no external provider endpoint is called. |
| `cargo run --offline --locked` | Passed; printed `capability spike ok`. |
| `cargo build --release --offline --locked` | Passed. |

The named commands worked from the existing cached dependencies; no network access was used by these checks. They are bounded crate probes, not product parity or application acceptance.

## QA2 reproduction commands and boundary

From repository root, set the same toolchain path and run:

```sh
export PATH="/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH"
cd docs/spike-r02
cargo fmt --all -- --check
cargo clippy --offline --locked --all-targets -- -D warnings
cargo test --offline --locked
cargo run --offline --locked
cargo build --release --offline --locked
```

Run the test command with normal host permissions if sandbox socket binding is denied; keep it offline. The ignored database probe is separate and must use a newly created disposable PostgreSQL instance only:

```sh
R02_PG_URL='<disposable PostgreSQL URL>' cargo test --offline --locked postgres_ddl_and_transaction_rollback -- --ignored
```

No disposable database was configured for Operations' integrated run, so this DB command was not rerun here. The source log reports Senior1's separate PostgreSQL 15 disposable-container pass; QA2 should independently decide whether that evidence suffices or repeat against its own disposable instance. Never use a shared or production database.

## Limits for QA disposition

The candidate documents a 269-package lock. Its active Darwin arm64 graph review covers 215 packages and lists MPL-2.0 and CDLA-Permissive-2.0 notices for follow-up; this is not legal signoff, vulnerability review, or verification of other platform graphs. No product schema/concurrency, persisted auth/session/revocation, production authorization, live provider, four-platform release, or end-to-end parity was tested. R01 maps are planned-test maps; their presence is not a fixture execution pass. QA2 independently reviews the exact integrated commit and decides all gate outcomes. No acceptance or release status is assigned here.
