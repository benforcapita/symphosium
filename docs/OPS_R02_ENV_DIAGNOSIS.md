# Operations R02 environment diagnosis

Project: Symphosium, S0/R02. Canonical linked worktree branch: `ops/symphosium-s0`. Starting revision: `2472570251c646a0f1787fcf76d46279e300de56`. Normative design: canonical `docs/PROJECT_LLD.md` revision 3, sections 11–12. No QA2 evidence was modified.

## Environment findings

- The default sandbox PATH omits the Rust toolchain bin directory. `cargo`, `rustc`, and `clippy-driver` were not found through PATH; `/opt/homebrew/bin/rustup` is present and reports active `stable-aarch64-apple-darwin`. Rust 1.97.1 tools and the `cargo-clippy` executable exist in `/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin/`; the rustup shim is not in `~/.cargo/bin`.
- Direct Cargo did not discover the `clippy` subcommand until that toolchain bin directory was prepended to PATH. With the one-command PATH adjustment, `cargo clippy --version` reported `clippy 0.1.97`; full spike Clippy then passed with `-D warnings`. No component install was needed.
- A bounded sandbox `curl` to `https://index.crates.io/config.json` failed DNS (`curl exit 6`, `Could not resolve host`). The same bounded check under normal approval escalation returned HTTP/2 200. This establishes sandbox DNS restriction for the checked request, not host-wide DNS unavailability.
- With host network and `CARGO_NET_RETRY=0`, `cargo generate-lockfile --manifest-path docs/spike-r02/Cargo.toml` succeeded and resolved 219 packages. `cargo build --locked` downloaded the needed open-source dependencies and completed successfully. Build output is ignored under `docs/spike-r02/target/`.

## Commands and results

Using the direct Cargo executable at the toolchain path, with the same bin directory prepended to PATH where Cargo subcommands are required:

| Command | Result |
| --- | --- |
| `cargo generate-lockfile --manifest-path docs/spike-r02/Cargo.toml` with `CARGO_NET_RETRY=0` and host-network escalation | Passed; lockfile resolved 219 packages compatible with Rust 1.97.1. |
| `cargo build --locked --manifest-path docs/spike-r02/Cargo.toml` with `CARGO_NET_RETRY=0` and host-network escalation | Passed; dev build finished. |
| `cargo fmt --manifest-path docs/spike-r02/Cargo.toml --all -- --check` | Passed. |
| `cargo clippy --offline --locked --manifest-path docs/spike-r02/Cargo.toml -- -D warnings` | Passed after PATH adjustment. |
| `cargo test --offline --locked --manifest-path docs/spike-r02/Cargo.toml` | Exit 0, but the package defines 0 tests; this is not behavioral test evidence. |
| `cargo run --offline --locked --manifest-path docs/spike-r02/Cargo.toml` | Failed at `src/main.rs:23`: the Askama HTML-escaping assertion failed. Later runtime probes in `main` did not execute. |
| `cargo build --release --offline --locked --manifest-path docs/spike-r02/Cargo.toml` | Passed; release build finished. |
| `cargo clippy --version` without toolchain bin on PATH | Failed to find subcommand. With toolchain bin prepended, it resolved the installed `cargo-clippy` executable. |

## Current blockers and boundaries

R02 is **not accepted**. The full candidate dependency set is now lockable and compiles on Darwin arm64, and the Clippy wiring has a reproducible local PATH remedy. However, its runtime capability program fails the Askama escaping assertion; there are zero Cargo tests, and probes after that assertion were not exercised. No PostgreSQL connection/migration, HTTP client, auth/session/CSRF, server route/SSE, full license audit, or four-target build was verified. The imported subset license inventory documents 18 target-specific registry manifests unavailable at the time of that scan; it is not a full audit of the new 219-package lockfile.

No Cargo component was installed, no paid service or live provider/model call was used, no QA gate was waived, and no push/release was attempted. Next engineering correction belongs with the spike owner; independent QA still reviews the exact integrated SHA. The prior QA2 report remains unchanged and is not a verdict on this environment run.
