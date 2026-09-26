# QA2 independent R05 foundation acceptance

**Project/repository:** Symphosium, `ops/symphosium-s0`  
**Integrated/published SHA tested:** `23b3f7f1a14eca8ed27b3812d9db10437149fb98`  
**QA checkout:** isolated clone `r05-qa-isolated`, detached at the SHA above  
**Authority:** `docs/PROJECT_LLD.md` revision 3 and `docs/DELIVERY_PLAN.md` R05.  
**Verdict:** **Accepted — R05 foundation only.**

This is an independent ticket verdict for R05, not a pass for R06+, R19, release, remote publication, external provider operation, scheduler dispatch, database persistence, authentication, or browser acceptance.

## Evidence

Canonical reference checkout was left unchanged; it remains at `be10a1b79df723d6d7612b5651c8522704dafb2e` with its pre-existing untracked design files. The QA clone resolved to the target SHA and was clean before and after tests.

With `/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin` prepended to `PATH`, from `r05-qa-isolated/rust`:

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | Pass. |
| `cargo clippy --all-targets --all-features --locked --offline -- -D warnings` | Pass. |
| `cargo test --all-features --locked --offline` | Pass: 14 library + 4 additional-review + 3 review-contract = **21 passed, 0 failed**; binary/doc tests 0. |
| `cargo build --release --locked --offline` | Pass. |
| `LINEAR_API_KEY=<synthetic> target/release/symphosium --i-understand-that-this-will-be-running-without-the-usual-guardrails --check WORKFLOW.example.md` | Exit 0: `valid workflow generation 1 tracker=linear`. |
| Same CLI command with `LINEAR_API_KEY` unset | Exit 1: `missing_linear_api_token`; synthetic key was not echoed. |

No live tracker/model endpoint, database, paid service, push, release, or source repair was used. Cargo was locked and offline.

## Acceptance assessment

- Configuration/workflow: typed config, front matter, aliases, secret-reference naming, policy validation, immutable snapshots, all-or-nothing last-known-good reload, and claim-scope fencing are covered by the passing foundation tests.
- External/no-DB foundation: `config::tests::external_memory_starts_without_database_or_linear_key` passed; the CLI `--check` validation is non-dispatching. No scheduler claim or provider request was made.
- Scope/reload: `reload::tests::env_scope_change_is_fenced_even_when_file_is_unchanged` and `review_contract::effective_assignee_scope_changes_when_env_changes` passed.
- Prompt errors: malformed Liquid validation/render paths passed with `template_parse_error:` and a stable sanitized workflow-prompt marker; workflow-load errors remain separately classified. The passing source test verifies malformed prompt text is not exposed.
- State policy: mixed map test passed: blank/non-string states, non-positive/non-integer/out-of-range limits are ignored; valid normalized duplicate states retain the deterministic last valid source-order value. This intentionally follows normative SPEC §5.3.5. Elixir schema rejects some invalid-map inputs, so this is a documented normative divergence, not a blanket parity assertion.
- Label/CLI regressions: normalized required-label deduplication, blank-label fail-closed behavior, regular-file/path/override validation, and port `0` handling passed through the included contract/unit tests.

## Cycle-2 findings

| Finding | Severity | QA2 result |
|---|---|---|
| S2-R05-001 scope/reload | High | Closed; target test suite passed relevant claim/environment scope tests. |
| S2-R05-002 per-state invalid entries | Medium | Closed; mixed invalid-map/duplicate test passed and SPEC divergence is documented above. |
| S2-R05-003 malformed Liquid context | Medium | Closed; validation and render use contextual sanitized parse errors, with separate workflow-load classification. |
| S2-R05-004 labels | Medium | Closed; review-contract regression passed. |
| S2-R05-005 CLI | Medium | Closed; independent positive/negative `--check` commands passed with secret-safe error output. |

No open Critical/High/Medium R05 defect remains. The prior failed repair count does not trigger CTO escalation because the same findings pass independent cycle-2 retest. No new low defect is recorded.

## Deferred limitations

Provider I/O/normalization, database schema/migrations/concurrency, runtime dispatch/retry/reconciliation, built-in tracker, auth/session/CSRF, agent tool authorization, worker isolation, SSH lifecycle, UI/browser, packaging targets, and full reference parity are intentionally deferred tickets. Their absence is not an R05 failure, and R05 evidence must not be reused as acceptance for them.

## Artifact hashes

| File at target SHA | SHA-256 |
|---|---|
| `rust/Cargo.lock` | `5c348753648d27147990339f3c908e044d1b6ae59f90c903c291cf6b8a3e156b` |
| `rust/Cargo.toml` | `ef89f2ebc55bd196074e7423e556b58de7e4df664adcdef756a87cf7bd76b3b2` |
| `rust/src/config.rs` | `a1cb0602b90bb2b2690f802be1c02e2fd00dc78a7bc98ffda58cf0df4bbacb09` |
| `rust/src/prompt.rs` | `478c16f52dce64750398e17a67faf85aa1f72ebb553af07cb08f53b5d0674d31` |
| `rust/src/reload.rs` | `28b8e0bd7eb665394ca91e52ffad43224d1df4f40fa72589cb09d9892916fa6b` |
| `rust/tests/review_additional.rs` | `c49beb7e6f8990d1e6a51ae85e25f06d9cc811c76debec2cc30d3cee33210d94` |

