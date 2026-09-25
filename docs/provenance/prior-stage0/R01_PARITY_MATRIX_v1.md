# R01 parity inventory v1 (discovery, not acceptance)

Source: `symphony-linear` at `be10a1b79df723d6d7612b5651c8522704dafb2e`. Authority: `PROJECT_LLD.md` revision 2. This maps SPEC sections 4–18 and Appendix A to planned Rust modules/tests. No Rust implementation exists in this artifact. `R` means SPEC required, `E` existing extension to retain, `O` optional/recommended upstream behavior selected by LLD. Every fixture below is a **proposed sanitized fixture** unless called existing; reference and Rust scenario outputs still need comparison.

| SPEC | Behavior and classification | Reference source/test | Rust target; named test | Fixture ID |
| --- | --- | --- | --- | --- |
| 4 domain | Issue fields, stable ID, normalization, attempts and retries (R) | `tracker/issue.ex`, `tracker/memory.ex`, `core_test.exs` | `tracker::issue`; `issue_normalization_contract` | F01 |
| 5 workflow | Discovery, YAML front matter, defaults/aliases, Liquid prompt and hooks (R) | `workflow.ex`, `config.ex`, `prompt_builder.ex`, `workspace_and_config_test.exs` | `config`, `workflow`; `workflow_parity_table` | F02 |
| 6 config | Resolution, last-good reload and preflight (R) | `config.ex`, `orchestrator.ex`, `workspace_and_config_test.exs` | `config`; `reload_preserves_active_scope` | F03 |
| 7 state | Claims, attempts, retries, recovery (R) | `orchestrator.ex`, `core_test.exs` | `orchestrator`; `stale_attempt_generation` | F04 |
| 8 scheduler | Selection, labels, limits, backoff, reconciliation, cleanup (R) | `orchestrator.ex`, `core_test.exs`, `orchestrator_status_test.exs` | `orchestrator`; `poll_reconcile_retry_contract` | F05 |
| 9 workspace | Safe path/identifiers, lifecycle, hooks, symlinks (R; population O) | `workspace.ex`, `path_safety.ex`, `workspace_and_config_test.exs`, `workspace_before_remove_test.exs` | `workspace`; `path_and_hook_contract` | F06 |
| 10 agent | Local app-server wire, approvals, timeout, token events (R) | `codex/app_server.ex`, `app_server_test.exs` | `agent::app_server`; `app_server_wire_contract` | F07 |
| 11 tracker | Read callbacks, errors, normalized fields (R); five native tool sets (E) | `linear/*`, `github/*`, `jira/*`, `asana/*`, `gitlab/*`, adapter tests | `tracker::{linear,github,jira,asana,gitlab,memory}`; `provider_contract_matrix` | F08–F12 |
| 12 prompt | Context, rendering, continuation/failure (R) | `prompt_builder.ex`, `core_test.exs` | `workflow::prompt`; `prompt_golden_contract` | F13 |
| 13 observability | Structured logs/accounting (R); dashboard/API/snapshots (E) | `status_dashboard.ex`, `tracker.ex`, `status_dashboard_snapshot_test.exs`, `observability_pubsub_test.exs` | `observability`, `web`; `status_and_api_contract` | F14 |
| 14 failure | Transport/retry/restart/interruption policy (R) | `orchestrator.ex`, `app_server.ex`, `core_test.exs` | `orchestrator`, `agent`; `failure_recovery_matrix` | F15 |
| 15 security | Path/secret/hook safety (R); PM auth/isolation (LLD) | `path_safety.ex`, `ssh.ex`, `workspace_and_config_test.exs` | `workspace`, `accounts`, `agent`; `secret_and_scope_boundary` | F16 |
| 16 algorithms | Startup/poll/reconcile/dispatch/worker exit (R) | `orchestrator.ex`, `cli.ex`, `core_test.exs`, `cli_test.exs` | `orchestrator`, `cli`; `lifecycle_trace_contract` | F17 |
| 17 validation | Offline matrix and real integration profile (R/O) | all `*_test.exs`; live suites exist | integration tests; `reference_scenario_runner` | F01–F20 |
| 18 checklist | Required conformance, recommended ops validation (R/O) | `elixir/Makefile`, `README.md` | CI gates; `release_gate_manifest` | F20 |
| Appendix A | SSH worker transport (E, fork requirement) | `ssh.ex`, `ssh_test.exs` | `agent::ssh`; `ssh_wire_and_secret_boundary` | F18 |

## Extension and fixture manifest

Version `v1`, UTF-8 JSON or text, synthetic IDs only. Do not copy access tokens, emails, hostnames, prompts, URLs containing secrets, or raw live responses. Each fixture gets `reference_sha`, `case_id`, `input`, `expected`, and `redaction_reviewed` metadata. Existing snapshot text under `elixir/test/fixtures/status_dashboard_snapshots/` is reference material, not yet certified sanitized. Fixture files have not yet been generated.

| IDs | Proposed cases | Source test anchor | Difference to make explicit |
| --- | --- | --- | --- |
| F01–F03 | Issue nulls/labels/blockers; workflow aliases/defaults; reload invalid YAML and scope switch | `core_test`, `workspace_and_config_test`, `cli_test` | LLD adds builtin and rejects scope change with claims; external default unchanged |
| F04–F07 | claim generation; retry sequence; path traversal/symlink/hook errors; JSON app-server approval/tool/timeout frames | `core_test`, `workspace_and_config_test`, `app_server_test` | Tokio failure policy must reproduce observable retry and cleanup |
| F08–F12 | Linear, GitHub, Jira, Asana, GitLab: selected scope, all pages, native ID and URL, labels, blockers, malformed/transport errors, credential-limited raw tool exposure | corresponding `*_adapter_test`, `extensions_test`, `dynamic_tool_test`, `gitlab_redirect_test` | Provider-specific normalization and tool advertisement cannot be flattened |
| F13–F17 | Prompt render/continuation; HTTP `/` and `/api/v1/*` shapes; token accounting; restart/failure trace; child secret denial | `core_test`, `status_dashboard_snapshot_test`, `orchestrator_status_test`, `app_server_test` | PM-enabled auth intentionally protects observability routes; disabled mode retains trusted-network behavior |
| F18–F20 | SSH launch/frames/cancel/env; CLI flags/error codes; packaging/offline gate manifest | `ssh_test`, `cli_test`, `elixir/Makefile` | Rust packaging requires new four-target proof; no equivalence inferred from Elixir binaries |

## Work still required before R01 review

Extract exact assertion-level reference scenarios and sanitized outputs for F01–F20; run reference offline tests; implement the same Rust harness; compare per-case results and document deviations. Live suites `*_live_e2e_test.exs` and `live_e2e_test.exs` are explicitly unrun. This inventory is traceability planning, not parity proof.
