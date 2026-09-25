# R02 bounded Rust capability spike v1

Source baseline `be10a1b79df723d6d7612b5651c8522704dafb2e`; LLD revision 2. No dependency downloaded, installed, built or license-audited. No feature implementation started. Reference root `LICENSE` is Apache-2.0 and `NOTICE` is present; dependency licenses remain unverified.

## Capability and dependency decisions pending lockfile proof

| Capability | Candidate | Spike proof required | Risk |
| --- | --- | --- | --- |
| Async HTTP/runtime | Tokio, Axum, tower | bounded channels, SSE disconnect, graceful shutdown | cancellation and panic recovery differ from OTP |
| PostgreSQL | SQLx + postgres | offline migrations, row locks, composite FK, reversible disposable down, DB outage | compile-time query metadata and target TLS linking |
| Templates/Markdown | Askama or MiniJinja; pulldown-cmark + ammonia | autoescape, safe links, raw HTML/image denial, no-JS forms | sanitizer/version interaction and CSP |
| Workflow | serde_yaml or maintained YAML parser; Liquid crate | aliases, front matter, strict Liquid parity | YAML edge cases/template filters differ from Elixir |
| Auth | argon2, password-hash, rand, cookie/session stack | password verification, revocation, CSRF, expiry, key rotation policy | session primitives may require first-party implementation |
| Providers | Reqwest, serde_json | paging, timeout, redirect, raw tool controls | five API-specific mappings and TLS roots |
| Logs | tracing, tracing-subscriber | safe fields, token accounting | secret redaction requires explicit review |

These are candidate crates, not selected or licensed dependencies. Verify exact versions, transitive SPDX/license expressions and maintenance from lockfile before adoption. Preserve root Apache-2.0 notices and attribution. Reject incompatible or unclear licenses through CTO review.

## Actual local checks

| Command | Result |
| --- | --- |
| `git -C <source> rev-parse HEAD` | `be10a1b79df723d6d7612b5651c8522704dafb2e` |
| `uname -sm` | `Darwin arm64` |
| `command -v cargo; command -v rustc` | neither present in shell PATH |
| `rustc --version; cargo --version` | both `command not found` |
| `command -v mix; command -v elixir; command -v psql` | none present in shell PATH |

Locked `cargo build`, `fmt`, `clippy`, `test`, SQL migration and Elixir `make all` therefore remain **unrun**, not passing. Toolchain acquisition must use approved free/local installation or existing CI; no paid spend. `rust-toolchain.toml` and `Cargo.lock` do not yet exist here. This spike proves environment availability and outlines proof cases; it does not prove selected crate compatibility.

## Four-target artifact risk

Required matrix: macOS arm64/x86_64, Linux arm64/x86_64. Current host proves only macOS arm64 inspection. macOS x86_64 needs compatible builder/target; Linux targets need musl/glibc, OpenSSL/rustls, SQLx and linker decisions. Bundle templates/static assets, migration files and license notices; verify each binary starts and serves external-only mode without DB, and PM mode with PostgreSQL. Keep existing Elixir/Burrito workflow until QA accepts Rust artifacts. No four-target packaging claim yet.

## Competence, availability and estimate

I can reason about Rust architecture and review asynchronous code, but have not demonstrated Rust compilation in this task because the toolchain is absent. No empirical throughput claim. Current competing assignment: this R01/R02 request; future availability beyond this turn is unknown. Reserve 20% capacity for Senior 2 review. R01 provisional 3–5 effort days and R02 2–4 remain **unvalidated**; fixture extraction and toolchain setup may increase them. Re-estimate after real reference and Rust checks, rather than promising a date.

Next owner: Operations checkpoint these two files on its integration branch; PM and Senior 2 review discovery gaps; QA2 independently plans parity cases. Do not start broad feature integration until stage-0 review.
