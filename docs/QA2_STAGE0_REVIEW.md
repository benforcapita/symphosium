# QA2 independent Stage-0 review — Symphony Rust S0

**Date:** 2026-09-25  
**Owner:** QA Engineer 2  
**Scope:** independent review of R01, R02, R04; planning gate only.  
**Source baseline:** `be10a1b79df723d6d7612b5651c8522704dafb2e`.  
**Integrated Rust SHA:** none. **Implementation acceptance:** not performed.

## Verdict

**Stage-0: Changes Required.** The discovery material supports planning but does not satisfy the specified fixture/spike evidence needed to open broad integration. This is not a failed code test, defect finding, or repair/retest cycle. No critical/high/medium implementation defect can be asserted because no Rust implementation was supplied.

| Criterion | Actual inspected evidence | Verdict | Missing/blocker / next owner |
|---|---|---|---|
| R01 traceability | `R01_PARITY_MATRIX_v1.md` maps SPEC 4–18 and Appendix A to planned modules/tests F01–F20, labels itself discovery/not acceptance, and identifies provider/native-tool differences. | Changes Required | No on-disk F01–F20 sanitized fixtures, exact assertion-level source anchors, reference captures, Rust harness, or per-case comparison/deviation record. Senior 1 must extract fixtures and anchors; Operations must checkpoint. |
| R02 executable bounded spike | `R02_RUST_SPIKE_v1.md` lists candidate crate families and risks; it correctly records that no dependency was downloaded/built/licensed/locked. | Changes Required | No selected versions, `Cargo.lock`, `rust-toolchain.toml`, license/SBOM or transitive review, bounded Cargo project/build, SQLx/template/auth feasibility, or four-target evidence. Senior 1 must run and preserve a bounded offline/reproducible spike. |
| R04 fixture inventory | `docs/R04-fixture-catalog.md` inventories 11 fixture files and 25 test modules, identifies inline adapter/app-server cases, prohibits live opt-ins, and distinguishes snapshots from runtime proof. | Review only | Catalog is useful, but does not create provider/app-server golden fixtures or runnable parity harness. Senior 1/R04 owner must supply extracted sanitized cases for R01. |
| R04 safe reference commands | `make e2e` is excluded because it sets a live Linear opt-in. `make all` is not offline because it invokes setup/dependency retrieval. Candidate offline commands are `mix format --check-formatted`, `mix lint`, and `mix test --cover --exclude live_e2e`, with every `SYMPHONY_RUN_*_LIVE_E2E` variable unset. | Safe classification confirmed; execution blocked | `mix`, `elixir`, `erl`, and `mise` are absent from PATH; no `deps/` or `_build/` cache observed. Do not call blocked commands passing. Authorized setup owner must provide an Elixir/Mix + locked dependency environment. |
| Installed Rust-toolchain claim | Directly inspected `/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin/`: `cargo 1.97.1`, `rustc 1.97.1`, `rustfmt 1.9.0-stable`, and `clippy-driver 0.1.97` execute. | Partially verified | `/Users/benblum/.cargo/bin/rustup` is absent; direct `cargo fmt` and `cargo clippy` report missing subcommands because the launcher/component wiring is absent. The R04 claim that `rustup run stable` succeeded is not reproducible here. This is not proof of a runnable Rust gate, and baseline contains no `Cargo.toml`. |
| Executable parity acceptance | No Rust directory/manifest, no integrated SHA, no migrated database, no reference test execution, and no Rust test harness. | Not executable / not accepted | Operations integration plus R01/R02/R04 remediation; later QA runs must compare actual fixture outputs at one integrated SHA. |

## Commands actually checked by QA2

```text
sed -n ... elixir/Makefile; sed -n ... elixir/mix.exs
ls -la /Users/benblum/.cargo/bin /Users/benblum/.rustup/toolchains
/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin/{cargo,rustc,rustfmt,clippy-driver} --version
```

Observed: Makefile confirms `all -> ci -> setup` and `setup -> mix setup`; `e2e` explicitly sets `SYMPHONY_RUN_LIVE_E2E=1`. `mix.exs` defines `lint` as `specs.check` plus `credo --strict`. No live-provider/model test was run and no dependency was installed.

## Release implication

Planning evidence may be checkpointed, but it cannot be represented as Rust parity or R19 acceptance. Proposed cargo package/test names in QA2 planning are placeholders until R02 establishes the actual Cargo package layout and test targets. The final R19 verdict remains contingent on one Operations-integrated SHA, all required executable gates, and zero open critical/high/medium defects.

## Operations checkpoint inputs

- `QA2_R19_ACCEPTANCE_MATRIX.md` (updated combined 12–22 QA-person-day estimate and Stage-0 distinction)
- `QA2_R19_DEFECT_LEDGER_TEMPLATE.md`
- `QA2_STAGE0_REVIEW.md`

## V2 static evidence review — supersedes the v1 fixture finding

**Review scope:** R01/R02 v2 and the eight `fixtures-v1` cases in Senior 1's isolated directory. This was a static inspection only: no Cargo formatting, compilation, test, dependency resolution, provider call, or model call was performed.

**Result:** **Stage-0 remains Changes Required.** The v2 evidence corrects the prior statement that no fixtures existed: eight checked-in, syntactically structured cases now exist. It does not satisfy exhaustive R01 fixture/harness scope, and R02 still lacks a resolved/locked successful build and license audit.

### Exact reviewed files and SHA-256

All below are under `/Users/benblum/.openmausbot/task-workspaces/68f56f75-eb0c-4f85-9c59-ee3f5c968b78/279d0ca2-388d-42ff-96ef-ae839cf89943/`.

| Artifact | SHA-256 | QA2 static result |
|---|---|---|
| `R01_PARITY_MATRIX_v2.md` | `7b62bd1245f3bda827038bb1afc08530ba4abcb4b83e22a6a33b54cae7a72568` | Accurate discovery status; fixture coverage explicitly incomplete. |
| `fixtures-v1/cases.json` | `197a0b916e024b18f119b3dbef1cfa3658e2bbfaf7bb0dcf40bcaf74a24bf2b8` | Eight structured cases; no credential-pattern match in static scan. |
| `fixtures-v1/check_cases.py` | `04ddaf0f3755221551efea3accae508eac83ca9d5a86e1f06de8827f984d768f` | Now requires `--reference-root` and checks `SPEC.md`; portable path correction is present. Structural only. |
| `R02_RUST_SPIKE_v2.md` | `61673006cf4c3ea333810b1ecee5620290d5b98109b544412147d2ed20eb68a7` | Correctly says no lockfile/build/full audit. |
| `spike-r02/Cargo.toml` | `466b50dbeb591e4e49b9f765834909c85a19dc72d5263057c87591cc65d58d7d` | Candidate ranges only; no resolved dependency evidence. |
| `spike-r02/rust-toolchain.toml` | `e7155b5437585c3c3acc3053b08c6936020e9e93cc343e47e7e25f0033418e84` | Pins 1.97.1/components but needs rustup/lock proof to be operational. |
| `spike-r02/src/main.rs` | `f47fdb79aa21e88bd4fcd722fefcdfb2df8d5efea26e7db5a87ecd5dbb9c402a` | Static capability composition only; unbuilt. |

### Criterion-level review

| Criterion | Evidence and fidelity | Verdict / remaining work |
|---|---|---|
| R01 fixtures and sanitization | Four scheduler cases and two SSH trace cases accurately reflect the named reference test scenarios at the supplied title anchors. Static credential-pattern scan of the fixture directory found no API-key, authorization, bearer, password, secret, token, or common token-prefix strings. This is a content review, not secret-scanner certification. | **Partial / Changes Required.** Provenance claims are reasonable for synthetic checked-in test values, but remaining F01–F20 areas are not extracted. |
| R01 assertion fidelity | `check_cases.py` verifies a test title at one line plus nonempty fields; it does not bind each expected value to its own assertion line or execute reference behavior. The two GitLab cases collapse a multi-assertion scenario: `gitlab_paging` omits `order_by`, `sort`, project-path, logged malformed-count, all-state and unsupported-state assertions; `gitlab_empty_states` uses the test-title anchor rather than the later exact empty-state assertion. | **Changes Required.** Add assertion-level anchors (possibly a list per case) and preserve every asserted observable, then extract the listed config/prompt/local-app-server/token/HTTP/provider/native-tool/retry/SSH-cancel-secret cases. Implement a reference/Rust result-comparison harness later; structural validation is not parity acceptance. |
| R02 probe scope | Manifest/source references Tokio, Axum, SQLx `PgPoolOptions`, Askama escaping, YAML, Liquid, Markdown-to-Ammonia, and Argon2 hashing. It does not statically demonstrate a SQL connection/migration/transaction, bounded channel/SSE/shutdown, Reqwest/provider timeout/redirects, tracing redaction, session-cookie/CSRF/revocation, Argon2 verification, or LLD-required remote-image/data-URL denial. The sanitizer assertion checks only `javascript:` after default cleaning. | **Changes Required.** Expand the eventual successful probe to each required behavior and record outputs; do not treat source composition or formatting as compilation. |
| R02 dependency/license/target proof | No `Cargo.lock`; candidates use broad compatible-version ranges. R02 records offline failure on uncached ammonia and crates.io DNS failure. Only Tokio, SQLx and Serde cached manifest licenses are reported; no resolved transitive graph/audit exists. Darwin arm64 direct tools only; no other package target evidence. | **Changes Required.** Do not retry the same network attempt without an environmental change. Obtain a complete authorized cache/connectivity, generate/review a lockfile, build, audit resolved licenses, and execute the target matrix evidence. |
| R04 direct-toolchain correction | QA2 directly executed `/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo --version` (`cargo 1.97.1`), `rustc --version` (`rustc 1.97.1`), direct `rustfmt --version`, and direct `clippy-driver --version`. `/Users/benblum/.cargo/bin/rustup` is absent; direct cargo reports no `fmt` or `clippy` subcommand because cargo-component shims are absent from this environment. | **Correct R04 wording:** Rust compiler/component binaries are installed outside PATH at the exact direct toolchain path; no rustup shim is verified; direct `cargo fmt`/`cargo clippy` gates are unavailable. This proves neither a Cargo project build nor a full Rust gate. |

No runtime pass, implementation pass, defect retest count, or release sign-off is created by this review. The latest PM-verified documentation base remains `bde3de0ef315298d64ad9cde673638b2abb8dff8`; reported v2 checkpoint `0bcac59ef61acf6ccb44b13ae995c93c904f9d89` has no verified checkout/manifest in this review.

## Canonical 38-runtime / 33-provider corpus review — 2026-09-25

**Reviewed checkpoint:** canonical documentation revision `0f33d80513223d90d87b3b8a572d21b0cd3cedfd`; reference source `be10a1b79df723d6d7612b5651c8522704dafb2e`.  
**Verdict:** **R01 Stage-0 remains Changes Required.** The corpus is a material improvement over the eight-case v2 review and fulfills structural traceability for 71 fixture records. It is not executable reference/Rust parity, an implementation pass, or a requirement to mechanically port the assertion index's 815 starts.

### Checks actually run

```text
python3 docs/fixtures-v1/check_cases.py --reference-root <baseline source>
# validated 38 fixture anchors and schemas; runtime parity untested
python3 fixtures-r01-provider-v1/check_cases.py --reference-root <baseline source>
# validated 33 provider fixture anchors and schemas; runtime parity untested
```

Static content review found only synthetic identifiers and named environment-variable references, not credential values. The assertion index is a discovery inventory (815 assertion starts across 202 tests); it is useful to locate missing behavior but is not an invented 815-test acceptance denominator.

### Exact artifact hashes

| Corpus artifact | SHA-256 |
|---|---|
| `docs/fixtures-v1/cases.json` | `845f0decc7cfc1ad248ff48b9c3c4dfa93ca00cc1a158fb110330d25bea4103f` |
| `docs/fixtures-v1/check_cases.py` | `eb8e5025be02e89adcfcec05b9163b4f0b881a1539d8a6253415041ba26e3757` |
| `docs/fixtures-v1/COMPLETENESS.md` | `b00143b8d3292a229352553d8fc6d0397120fad023cd8caeb03b76a2a05c2fd2` |
| `docs/fixtures-v1/reference_assertions.json` | `64be57e0a1b2d31a0ceb223d8aee2fa41ef131217e2711043632699c3e9383f0` |
| `fixtures-r01-provider-v1/cases.json` | `c4ac83b1424a528b1b9fcfaf1737df95b20f1d9bfcc8682f3bd35a94d6c47a95` |
| `fixtures-r01-provider-v1/check_cases.py` | `531fc01a6b96231b260f93b19f347ce780c640e05f73007787be87306bf38d99` |
| `fixtures-r01-provider-v1/COVERAGE_MANIFEST.md` | `38450b045ab89a829c16d0e7ba36be6f822e27cb4e0edda4c15a1076789f20ed` |

### Defect ledger entries and finite corrections

| Defect ID | Severity | Corpus case / reference anchor | Expected vs. actual | Required correction |
|---|---|---|---|---|
| QA2-R01-FX-001 | Medium | Runtime `config_aliases`; `elixir/test/symphony_elixir/workspace_and_config_test.exs:1201` | Reference asserts the provider `api_key`, the top-level alias, secret-environment name, and complete normalized provider map. Fixture input omits `api_key`; expected omits the alias and normalized map. | Add synthetic `api_key` input plus expected top-level alias and complete provider map (including `assignee: null`), retaining no credential value. Add assertion-line references, not only the test header. |
| QA2-R01-FX-002 | Medium | Runtime `app_server_partial_lines`; `elixir/test/symphony_elixir/app_server_test.exs:1187` | Reference sends a 1,100,000-byte JSON line, requires no completion before newline, then completes only after specific thread/turn frames. Fixture expresses byte size and only `run: ok`. | Represent the fragmented frame sequence, pre-newline non-completion, newline boundary, thread/turn IDs, and terminal event/result. This is required to prevent a Rust framing implementation from passing while incorrectly processing partial input. |
| QA2-R01-FX-003 | Medium | Provider `asana_normalize_fields` at `asana_adapter_test.exs:101` and `gitlab_normalize_fields` at `gitlab_adapter_test.exs:116` | Reference asserts concrete native-reference shapes and normalized title/description/state/URL/assignee/labels/blockers/timestamps. Fixtures use coarse `preserves_native_ref: true` and omit most asserted fields. | Replace boolean preservation summaries with synthetic complete native-ref objects and required normalized fields; include blank/terminal variants where the same source test asserts them. |
| QA2-R01-FX-004 | Medium | Provider paging/refresh cases: `jira_paging_filter_malformed` (`jira_adapter_test.exs:247`), `jira_refresh_batch_order_scope` (`:317`), `asana_paging_sections_malformed` (`asana_adapter_test.exs:145`), `asana_refresh_scope_404` (`:190`) | These cases encode behavioral booleans but not the concrete request path/query/batch boundaries/returned order that a port must reproduce. The manifest also identifies provider request validation as unrepresented. | Add a bounded request/response transcript projection per case: path, query/cursor/offset, batch grouping, ordered IDs and omitted IDs; maintain synthetic values. |

No Critical/High implementation defect is asserted: these are Medium traceability-fixture defects because they could permit materially incomplete required compatibility behavior to be accepted at the Stage-0 gate. Repair/retest count is zero; no implementation repair was tested.

### Necessary missing behavior categories (not a mechanical assertion-index requirement)

The corpus itself accurately declares outstanding categories. Before R01 can be accepted for Stage-0, retain an explicit fixture/map outcome for: Linear normalization/cursor paging/refresh/assignee filtering; configuration validation and secret-environment declaration for every provider; provider raw-field/date/description edge cases; malformed/non-JSON/native-tool error matrices; request path/validation; tracker binding/reload session snapshot; SSH cancellation/forwarded-secret isolation and frame limits; app-server approvals/MCP/parser/timeout/tool-failure paths; complete observability payloads/accounting monotonicity; and LLD-only scope-switch/worker identity boundaries. A categorized map with reasoned non-applicability is sufficient where the reference has no analogue; no one needs to fabricate 815 separate tests.

**Unchanged blocker:** R02 has no resolved lockfile, successful build, complete license audit, or network/cache change. No runtime parity conclusion can be drawn until those conditions and a Rust comparison harness exist.
