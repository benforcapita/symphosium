# R02 bounded feasibility result v3 — Symphosium

Authority: `PROJECT_LLD.md` revision 3 sections 11–12. Reference source `be10a1b79df723d6d7612b5651c8522704dafb2e`. Isolated artifact `spike-r02/`; no edit to source or integration checkout. No paid service, live provider or model call. Full R02 gate is **not satisfied**.

## Commands and actual outcomes (Darwin arm64, direct Rust 1.97.1 toolchain)

| Command | Outcome |
| --- | --- |
| `CARGO_NET_RETRY=0 RUSTC=<absolute rustc> <absolute cargo> generate-lockfile` in full `spike-r02` | Exit 101: `ammonia` dependency resolution failed; `Could not resolve host: index.crates.io` for config.json. No full lockfile. |
| `<absolute cargo> generate-lockfile --offline --manifest-path offline-min/Cargo.toml` | Exit 0: locked 140 registry packages (Tokio 1.53.1, SQLx 0.8.6, Serde 1.0.229 direct pins). |
| `<absolute cargo> build --offline --locked --manifest-path offline-min/Cargo.toml` | Exit 0, dev build finished in 15.60s. |
| `<absolute cargo> test --offline --locked --manifest-path offline-min/Cargo.toml` | Exit 0: `bounded_channel_backpressure` passed, 1/1. |
| `<absolute cargo> run --offline --locked --manifest-path offline-min/Cargo.toml` | Exit 0: printed `tokio/sqlx probe`; validated bounded channel send/receive and synthetic PostgreSQL URL parsing. No DB connection. |
| `<absolute cargo> build --release --offline --locked --manifest-path offline-min/Cargo.toml` | Exit 0, release build finished in 17.55s. |
| `<absolute rustfmt> --edition 2024 --check offline-min/src/main.rs` | Exit 0. |
| `<absolute cargo> clippy --offline --locked --manifest-path offline-min/Cargo.toml` | Exit 101: no `clippy` Cargo subcommand/shim. |
| `RUSTC=<absolute clippy-driver> RUSTFLAGS='-D warnings' <absolute cargo> check --offline --locked ...` | Exit 101: libc/zerocopy build scripts cannot parse `clippy 0` as rustc version. This workaround is invalid; no lint pass. |
| `python3 offline-min/audit_licenses.py --cargo-cache <local cache> --output offline-min/license_inventory.json` | Exit 0: 140 locked registry packages inventoried; 122 cached manifests have license declarations, 18 platform-specific package manifests missing locally. This is an inventory, not a full compatibility/security audit. |

Direct tool paths: `/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin/{cargo,rustc,rustfmt,clippy-driver}`. `offline-min/Cargo.lock` is a genuine lockfile for the **subset only**; it does not resolve Axum, templates, YAML/Liquid, Markdown sanitizer, password hash, HTTP provider client or sessions. The main `spike-r02/Cargo.toml` remains candidate dependencies with no lockfile/build. No PostgreSQL server/connection, CSRF/session implementation, Reqwest/provider timeout, Markdown URL/image policy or four-target test ran.

## License inventory limits

The locally cached manifests include 70 `MIT OR Apache-2.0`, 14 `MIT`, 18 `Unicode-3.0`, and other permissive expressions recorded per package in `offline-min/license_inventory.json`. Eighteen locked registry entries have no cached manifest in this environment (Windows/Redox/WASI packages); their licenses are **unverified** here. This scan reads manifest declarations only; it does not verify notice files, vulnerabilities, or the full eventual dependency graph. No claim of final license approval.

## Remaining bounded probes and unblock condition

Once crates.io DNS or an authorized complete offline cache is available, resolve/lock the main spike, then compile and run Axum route/SSE, SQLx disposable PostgreSQL transaction/migration, template escaping, YAML and Liquid compatibility, Markdown raw HTML/javascript/data/remote-image denial, Argon2 hash/verify, session cookie/CSRF/revocation and Reqwest timeout/redirect probes. Use synthetic local endpoints and disposable DB only. Audit all resolved transitive licenses and run fmt, lint, tests and release locked build. The current subset proves Tokio/SQLx/Serde toolchain feasibility on Darwin arm64 only.

Revised R02 effort: **3–6 person-days**, from 2–5, to account for missing dependency cache, boundary probes and license review; no elapsed commitment while DNS and PostgreSQL remain unavailable. Rust competence evidence now includes one successful locked subset build/test/release, but not the full stack. Availability remains this assigned work only, with 20% reserved for counterpart review. Next owner PM for Operations checkpoint and QA2 review. No broad feature integration or R02 acceptance claim.
