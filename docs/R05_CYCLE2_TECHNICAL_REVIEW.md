# Symphosium R05 cycle-2 technical re-review

**Verdict:** both remaining Medium findings technically closed at published integrated SHA `23b3f7f1a14eca8ed27b3812d9db10437149fb98`; independent QA2 verdict remains pending. Code commit `aa4a693ae04597fd9faa288867988de9bb3f053e` differs from target only in `docs/OPS_R05_CYCLE2_CHECKPOINT.md` and `docs/OPS_R05_CYCLE2_SHA256.txt`. I cloned the canonical repository locally into the isolated `r05-cycle2-review` checkout and detached at the exact target; no other worktree was edited.

## Findings and dispositions

| ID | Disposition | Evidence |
| --- | --- | --- |
| **S2-R05-001 High** scope/reload | **Technical closure retained.** | Effective org/project/assignee values remain captured in immutable snapshots; unchanged-file env scope changes with an active claim remain fenced. `reload::env_scope_change_is_fenced_even_when_file_is_unchanged` and `review_contract::effective_assignee_scope_changes_when_env_changes` pass. No cycle-2 edit touched these modules. |
| **S2-R05-002 Medium** per-state policy | **Technically closed.** | `config.rs` now parses a YAML mapping and skips blank/non-string states and non-positive, non-integer or out-of-range limits while preserving valid values. `review_additional::invalid_per_state_limit_is_ignored_per_spec` and `invalid_state_entries_are_skipped_and_normalized_duplicates_use_last_valid` pass. Normalized duplicates have a deterministic last-valid-value rule; invalid later values do not erase a valid earlier value. |
| **S2-R05-003 Medium** Liquid error context | **Technically closed.** | Both `prompt::validate` and `render` use one `parse_template` path and classify malformed syntax with `template_parse_error:` and a sanitized `template="<workflow prompt>"` marker. The two `review_additional` tests pass; a separate review-only synthetic marker test also confirms the malformed prompt body is absent from the error. Workflow-file load errors retain a distinct class. |
| **S2-R05-004 Medium** labels | **Technical closure retained.** | Normalized, stable-order deduplication and blank fail-closed behavior remain; prior review-contract regression passes. |
| **S2-R05-005 Medium** CLI | **Technical closure retained.** | Regular-file/path/override validation remains; CLI `--check` succeeds with a synthetic key and fails with `missing_linear_api_token` without it. Port `0` remains valid per reference `cli.ex:165,173`. The deliberate `RuntimeUnavailable` path outside `--check` belongs to R12 dispatch, not an R05 failure. |

### SPEC versus Elixir behavior

SPEC §5.3.5 explicitly says invalid `max_concurrent_agents_by_state` entries are ignored. Elixir's `config/schema.ex:367–384` validates the entire map and reports errors for blank or non-positive entries, as its helper test shows. Rust cycle 2 follows the normative language-independent SPEC. This is a **documented semantic difference**, not a parity claim; QA2 should exercise valid and invalid mixed maps on the integrated SHA. The chosen normalized-duplicate last-valid result is deterministic and covered by a test, though SPEC does not prescribe duplicate precedence. No new architectural requirement is proposed.

## Independent commands and results

Verified `/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustc --version` = `rustc 1.97.1 (8bab26f4f 2026-07-14)` and corresponding Cargo = `1.97.1 (c980f4866 2026-06-30)`, matching `rust/rust-toolchain.toml`. With that bin directory prepended to `PATH`, from isolated `r05-cycle2-review/rust`:

- `cargo fmt --all -- --check` — pass.
- `cargo clippy --all-targets --all-features --locked --offline -- -D warnings` — pass.
- `cargo test --all-features --locked --offline` — pass: 14 unit, 4 additional-review, 3 prior review-contract tests; **21 passed, 0 failed**.
- `cargo build --release --locked --offline` — pass.
- `LINEAR_API_KEY=synthetic-token target/release/symphosium --i-understand-that-this-will-be-running-without-the-usual-guardrails --check WORKFLOW.example.md` — exit 0, `valid workflow generation 1 tracker=linear`.
- Same command with `env -u LINEAR_API_KEY` — exit 1, `missing_linear_api_token`.
- Extra isolated test `R05_CYCLE2_SECRET_REPRO.rs`, temporarily installed as `rust/tests/review_secret.rs`: `cargo test --locked --offline --test review_secret` — **1 passed**. The test was removed from the clone after execution; clone status is clean.

I inspected the full cycle-2 Rust diff (two modules plus the additional-review tests) and confirmed only two Ops evidence docs differ between code commit and published target. There were no source fixes, live provider/model calls, paid services, push, or release actions from this review.

## Contract readiness and next gate

The five Senior2 technical findings are closed for this SHA. R05's foundation interfaces are stable enough for R06/R07 implementation preparation and integration **subject to PM's dependency and QA gates**. The port is still a foundation: runtime dispatch, provider I/O, database and later auth behavior are not proven by this review. PM routes this exact SHA to independent QA2; only QA2 may issue the ticket verdict. This cycle-2 review found no remaining instance of S2-R05-002 or -003, so the two-failed-cycle CTO escalation condition is not triggered. Any new failure should be recorded with its exact reproduction and severity under the normal defect process.
