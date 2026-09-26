# QA2 R09 repair1 retest — publication copy

Exact QA source retained at `docs/provenance/QA2_R09_REPAIR1_RETEST.md`. Source SHA-256: `55dbcdc0552057dd0e7be26594785e6ab8c078f8d4ebadbbdab4cd14d4b8403e`.

# QA2 R09 repair1 independent runtime retest

**Repository:** `git@github.com:benforcapita/symphosium.git`
**Reviewed revision:** `3fa6277cef63e991d7cbb27c872729c39b72e9e3` (detached, clean checkout)
**Toolchain:** Rust/Cargo 1.97.1, macOS arm64
**Scope:** security repairs `S2-R09-001`, `S2-R09-002`, and `S2-R09-003` only.

## Results

| Command | Result |
| --- | --- |
| `cargo test --manifest-path rust/Cargo.toml --locked --offline --test github_security` | Pass: 3 passed. |
| `cargo test --manifest-path rust/Cargo.toml --all-features --locked --offline` | Pass: 28 passed. |
| `cargo fmt --manifest-path rust/Cargo.toml --all -- --check` | Pass. |
| `cargo clippy --manifest-path rust/Cargo.toml --all-targets --all-features --locked --offline -- -D warnings` | Pass. |
| `cargo build --manifest-path rust/Cargo.toml --release --locked --offline` | Pass. |

Cargo's bin directory was explicitly placed on `PATH`, because this sandbox's default
`PATH` does not include it. The first sandboxed security-test attempt compiled but
could not bind the synthetic loopback listeners (`Operation not permitted`); the same
unchanged, offline command was rerun outside that listener restriction and passed.
No live provider, model endpoint, real token, or paid service was used.

## Verdict

**Pass** for the three bounded R09 security-repair findings at this exact revision.
No new defect was observed.

This is not full R09 acceptance or a release verdict. Linear, Jira, Asana, and GitLab
adapters; provider-native tools and authorization; complete GitHub reference/fixture
coverage; and immutable-generation/claim-lease adapter-factory bindings remain open.
Relevant subsequent changes invalidate this result.
