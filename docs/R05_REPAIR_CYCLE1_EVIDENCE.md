# Symphosium R05 repair cycle 1 evidence — 2026-09-26

Source integration checkout was read at `3d1575da917587c9795090f0dc8e51f4de1a990d` (original R05 code checkpoint `f64fa76ca47becb426defaf13d7925d1665b1cb7`). Repair branch `s1/r05-repair` is an isolated local clone at `/Users/benblum/.openmausbot/task-workspaces/68f56f75-eb0c-4f85-9c59-ee3f5c968b78/279d0ca2-388d-42ff-96ef-ae839cf89943/symphosium-r05-repair`. No commit or push. Patch `R05_REPAIR_CYCLE1.patch` in its parent directory has SHA-256 `0a93a0bbab73a52ea36804d997440c88b2d796cb79527ec5fd7fe0fa64908c11`.

## Changes against Senior2 findings

- S2-R05-001: each snapshot captures an effective resolved kind/org/project/assignee scope. Same-file reload re-evaluates environment references; active claims fence changed scope. Claim snapshots retain prior credentials. Deterministic environment-injection test covers changed assignee and project with an active claim.
- S2-R05-002: typed settings now retain worker SSH limits, hooks, Codex approval and sandbox policies/timeouts, observability, server, and normalized per-state concurrency. Invalid values fail validation. Existing `codex_command` remains for downstream compatibility.
- S2-R05-003: default prompt contains the reference Body and missing-description fallback. Tests cover present/absent description, nested JSON field, ISO timestamp, strict unknown variable, and parse error.
- S2-R05-004: required labels trim/lowercase and deduplicate in stable order; blank remains fail-closed.
- S2-R05-005: CLI validates nonblank logs root, port range, regular workflow file, and expands paths. `StartupContext` retains overrides. Port zero is accepted deliberately because `elixir/lib/symphony_elixir/cli.ex` accepts `port >= 0` for an ephemeral bind; this differs from the review's suggestion to reject zero.

## Checks actually run

Toolchain: `/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustc --version` = `rustc 1.97.1 (8bab26f4f 2026-07-14)`; this matches `rust/rust-toolchain.toml`.

With that bin directory prepended to PATH, all from the isolated checkout:

- `cargo fmt --manifest-path rust/Cargo.toml --all -- --check`: pass.
- `cargo clippy --manifest-path rust/Cargo.toml --all-targets --all-features --locked --offline -- -D warnings`: pass.
- `cargo test --manifest-path rust/Cargo.toml --all-features --locked --offline`: pass, 14 unit and 3 copied Senior2 reproduction tests; zero failed.
- `cargo build --manifest-path rust/Cargo.toml --release --locked --offline`: pass.
- `LINEAR_API_KEY=synthetic-token rust/target/release/symphosium --i-understand-that-this-will-be-running-without-the-usual-guardrails --check --logs-root ./logs --port 0 rust/WORKFLOW.example.md`: exit 0, `valid workflow generation 1 tracker=linear`.
- Same CLI `--check` with no key: exit 1, `missing_linear_api_token`.
- Same CLI without `--check` and with synthetic key: exit 1, `runtime dispatch is not implemented in R05; use --check to validate WORKFLOW.md`.
- `git diff --cached --check`: pass.

No live tracker/model calls, PostgreSQL, scheduler dispatch, remote push, integration checkpoint, Senior2 re-review, or QA2 verdict. Ops must import/checkpoint this patch first, then Senior2 and QA2 check the exact integrated SHA. Suggested remaining effort: Operations import/checkpoint 0.5–1 hour, Senior2 re-review and QA2 independent tests 2–4 hours combined, plus 20% engineering review reserve. Any review rework is extra and cannot be estimated from this patch alone.
