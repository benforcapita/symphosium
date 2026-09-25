# R01 runtime/config/observability/security gap map — repair cycle 1

Source baseline `be10a1b79df723d6d7612b5651c8522704dafb2e`; LLD revision 2. This map closes the *discovery mapping* omission. Each row is a checked-in offline reference scenario and a planned Rust test. It is not executed parity. The `fixtures-v1/reference_assertions.json` index gives assertion-start anchors; normalized cases with `assertion_anchors` are required before a case is used as a golden comparison. Provider mapping is assigned to Mid1 and is excluded here.

| Area / required behavior | Exact reference scenario anchor | Planned Rust test/module | Evidence still needed |
| --- | --- | --- | --- |
| Config defaults/validation | `core_test.exs:4` | `config::defaults_validation` | normalized defaults/error fixture |
| WORKFLOW discovery/front matter | `core_test.exs:169,193,201,209` | `workflow::discovery_frontmatter` | normal/unterminated/non-map cases |
| Env reference and alias rules | `workspace_and_config_test.exs:1169,1201,1274` | `config::env_alias_contract` | FX-001 corrected fixture plus env cases |
| Per-state concurrency | `workspace_and_config_test.exs:1301` | `config::per_state_limits` | normalized limits fixture |
| Policy keys/sandbox resolution | `workspace_and_config_test.exs:1372,1418,1485,1542` | `config::sandbox_policy_contract` | exact policy cases |
| Last-good reload | `extensions_test.exs:86` | `workflow::last_good_reload` | compare reload transcript |
| Prompt Liquid/datetime/nested values | `core_test.exs:1305,1327`; `workspace_and_config_test.exs:1594` | `workflow::prompt_contract` | nested structure fixture and exact render |
| Workspace collision/path/symlink | `workspace_and_config_test.exs:83,171,199,235,262,1533` | `workspace::containment_contract` | path outcome matrix |
| Hooks create/remove/failure/timeout | `workspace_and_config_test.exs:865,903,928,953` | `workspace::hook_lifecycle_contract` | exact call/cleanup transcript |
| Scheduler dispatch/revalidation | `workspace_and_config_test.exs:682,720,741,763,789,810,837` | `orchestrator::dispatch_contract` | selected order and negative gates |
| Reconciliation/terminal cleanup | `core_test.exs:478,541,618,685,763,803,850,897` | `orchestrator::reconcile_contract` | stop-before-cleanup, absent issue, reassignment |
| Retry/backoff/stale timer | `core_test.exs:1022,1062,1102,1141,1181` | `orchestrator::retry_contract` | timing and generation assertions |
| Local wire framing/timeout/parser | `app_server_test.exs:79,1187,1326` | `agent::app_server::wire_contract` | FX-002 corrected frame transcript; malformed lines |
| Approval/input/MCP | `app_server_test.exs:275,354,419,482,619,718,788` | `agent::app_server::approval_contract` | allow/deny input cases |
| Dynamic tools | `app_server_test.exs:858,959,1081` | `agent::app_server::tool_contract` | exact envelope/error frames |
| Local child secret env | `app_server_test.exs:1397` | `agent::app_server::secret_env_contract` | denied names/profile inheritance |
| SSH parse/launch/framing | `ssh_test.exs:6,26,46,70,90,105,135,160`; `app_server_test.exs:1499` | `agent::ssh::transport_contract` | binary/line output and cancellation |
| Token accounting cumulative/last | `orchestrator_status_test.exs:203,278,472,560,634` | `observability::token_accounting` | monotonic totals and rate payloads |
| Snapshot retry/poll/status | `orchestrator_status_test.exs:717,757,802,854,903,975` | `observability::snapshot_contract` | exact snapshot and stalled-worker states |
| HTTP state/issue/refresh/errors | `extensions_test.exs:242,381,417,579` | `web::observability::api_contract` | complete JSON payloads and invalid-host results |
| CLI flags/path/startup | `cli_test.exs:8,46,58,82,101,114,128` | `cli::startup_contract` | full flag/error fixture |

## LLD-only cases with no reference analogue

`fixtures-v1/lld_only_tests.json` separately defines `pre_newline_no_completion` (stronger framing boundary), `scope_switch_with_claims`, and `separate_worker_secret_identity`. Other LLD-only acceptance cases remain SQLx transaction integrity, PM auth/CSRF/session revocation, scoped builtin tools, database outage failure, and run projection. These belong to R05–R19 implementation/QA; they must never be described as passing reference fixtures. SSH cancellation/secret forwarding require new Rust/LLD tests even where the reference has a launch case. This map is for stage-0 planning, not a substitute for executable evidence.
