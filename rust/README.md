# Symphosium Rust foundation

This R05 package validates `WORKFLOW.md`, normalizes typed tracker settings, renders compatible prompts, and exposes a claim-aware immutable configuration generation. It does not poll trackers or dispatch agents yet. External tracker configuration can load without PostgreSQL.

From `rust/`, run `cargo run -- --i-understand-that-this-will-be-running-without-the-usual-guardrails --check WORKFLOW.example.md`. `--check` validates config and exits without starting a scheduler. Omitting `--check` reports that dispatch is not implemented in R05. No database URL is needed for external tracker settings.

`--logs-root` and `--port` are validated and retained in `StartupContext` for the later runtime. The CLI expands the workflow and logs paths, requires a regular workflow file, and accepts port `0` as the reference CLI does. The external Linear example requires `LINEAR_API_KEY`; use a synthetic value for offline validation.

The configuration model retains hooks, SSH worker limits, per-state concurrency, Codex policies and timeouts, observability, and server settings. Claim leases pin a resolved scope and credentials; an environment-backed scope change is rejected while a claim is active.

Production implementation gates remain in later tickets. See root `SPEC.md` and `docs/PROJECT_LLD.md` revision 3.
