# Symphosium Rust foundation

This R05 package validates `WORKFLOW.md`, normalizes typed tracker settings, renders compatible prompts, and exposes a claim-aware immutable configuration generation. It does not poll trackers or dispatch agents yet. External tracker configuration can load without PostgreSQL.

From `rust/`, run `cargo run -- --i-understand-that-this-will-be-running-without-the-usual-guardrails --check WORKFLOW.example.md`. `--check` validates config and exits without starting a scheduler. Omitting `--check` reports that dispatch is not implemented in R05. No database URL is needed for external tracker settings.

Production implementation gates remain in later tickets. See root `SPEC.md` and `docs/PROJECT_LLD.md` revision 3.
