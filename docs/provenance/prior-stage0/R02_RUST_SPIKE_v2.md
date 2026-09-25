# R02 executable-spike correction v2

Base source revision `be10a1b79df723d6d7612b5651c8522704dafb2e`; LLD revision 2. New bounded project: `spike-r02/` with Rust 1.97.1 toolchain pin and capability probes for Tokio, Axum, SQLx PgPoolOptions, Askama escaping, YAML, Liquid, Markdown sanitization and Argon2. It is **not built** and has no lockfile. Candidate versions are ranges; they are not adopted project dependencies.

## Executed commands/results

| Command | Result |
| --- | --- |
| `/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustc --version` | `rustc 1.97.1 (8bab26f4f 2026-07-14)` |
| matching absolute `cargo --version` | `cargo 1.97.1 (c980f4866 2026-06-30)` |
| `cargo +1.97.1 generate-lockfile --offline` using absolute cargo | failed: direct cargo does not handle rustup `+toolchain` syntax; corrected invocation below |
| `RUSTC=<absolute rustc> <absolute cargo> generate-lockfile --offline` | failed: no matching package `ammonia` in local crates.io index |
| same command without `--offline` | crates.io index DNS failure (`Could not resolve host: index.crates.io`); interrupted after repeated warnings |
| `python3 fixtures-v1/check_cases.py` | passed eight structural cases; no runtime execution |

License check of **locally cached direct candidates only**: Tokio 1.53.1 reports MIT; SQLx 0.8.6 and Serde 1.0.229 report `MIT OR Apache-2.0` in their cached Cargo.toml. Root source is Apache-2.0 with NOTICE. This does not verify the other direct crates or transitive license graph, which needs a resolved lockfile and audit. No locked build, SQL connection, template render, auth hash, lint, test, or four-target artifact claim is possible yet. Current host is Darwin arm64; macOS x86_64 and both Linux targets remain untested.

Evidence corrects v1's PATH-only inference: Rust toolchain is installed outside PATH. Network resolution and missing cached crates now block dependency resolution. Free setup can resume when crates.io is reachable or an authorized complete cache is available. I have demonstrated Rust tool invocation and source composition, not a successful Rust build. Availability remains this assigned work only; reserve 20% for Senior 2 review. Revised effort: R01 **4–7 person-days** (was 3–5) for exhaustive assertion extraction and harness, R02 **2–5 person-days** (was 2–4) including dependency resolution/audit once network works; elapsed ranges cannot be committed while blocked. No paid spend, live provider/model tests or broad feature implementation.
