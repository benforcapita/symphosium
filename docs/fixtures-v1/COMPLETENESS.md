# R01 fixture completeness, v1

Source `be10a1b79df723d6d7612b5651c8522704dafb2e`; LLD revision 2. `cases.json` has 38 sanitized assertion-level cases from checked-in offline tests. `check_cases.py --reference-root <source checkout>` verifies exact test-line anchors and case structure. `build_reference_index.py` inventories assertion starts across the named non-provider offline test files into `reference_assertions.json`, redacting secret-related assertion text. The index covers all matching tests in those files, including cases still awaiting portable input/output normalization. These tools do **not** execute Elixir or Rust. `R01_PARITY_MATRIX_v1.md` maps every SPEC 4–18/Appendix A section at module level. The F IDs below retain its proposed fixture families; this table identifies concrete coverage and gaps. Provider extraction belongs to Mid1; integration/review must merge those cases without duplicate ownership.

| Family | Concrete case IDs present | Unresolved assertion-level coverage / owner |
| --- | --- | --- |
| F01 domain | sort_priority_age, required_labels_casefold | Issue null fields, normalized blocker object / Senior1 |
| F02 workflow | config_aliases, config_legacy_env_literal, prompt_liquid_issue_attempt, prompt_liquid_datetimes, cli_default_workflow | Full front matter defaults, filter parity, malformed YAML variants / Senior1 |
| F03 reload | reload_last_good | Scope switch with active claims is LLD-only; Rust test design pending / Senior1 |
| F04 attempts | retry_normal_exit, retry_abnormal_exit, stale_retry_timer | Stale completion generation is LLD-only; no reference assertion / Senior1 |
| F05 scheduling | sort_priority_age, provider_blocked, required_labels_casefold, closed_blocker_ready, refresh_coalesces, reconcile_nonactive, reconcile_terminal | Per-state/host concurrency and missing issue reconciliation / Senior1 |
| F06 workspace | hook_lifecycle, hook_failure_cleanup, workspace_symlink_escape | Root canonicalization, collision keys, hook timeout/large output / Senior1 |
| F07 local app-server | app_server_partial_lines, app_server_input_blocker, app_server_dynamic_tool, app_server_unsupported_tool, app_server_secret_env | Exact JSON frames, approvals, MCP elicitation, timeout, side output, parser error, tool failure / Senior1 |
| F08 Linear | none | All scope/paging/normalization/error/native tool cases / Mid1 |
| F09 GitHub | none | All scope/paging/normalization/error/native tool cases / Mid1 |
| F10 Jira | none | All scope/paging/normalization/error/native tool cases / Mid1 |
| F11 Asana | none | All scope/paging/normalization/error/native tool cases / Mid1 |
| F12 GitLab | gitlab_paging, gitlab_empty_states | Mid1 owns remaining GitLab cases and reconciliation / Mid1 |
| F13 prompt | prompt_liquid_issue_attempt, prompt_liquid_datetimes | Nested date/map and continuation/error behavior / Senior1 |
| F14 observability | token_total_preferred, token_last_only_ignored, http_method_errors, http_unknown_route, http_unavailable_refresh, http_snapshot_timeout | Full `/api/v1/state`, issue, refresh payloads and token monotonicity / Senior1 |
| F15 failure | retry_abnormal_exit, stale_retry_timer, reconcile_nonactive, reconcile_terminal | Crash/restart and outage failure matrix / Senior1 |
| F16 security | workspace_symlink_escape, app_server_secret_env | SSH secret isolation, hook environment, production identity boundary (LLD-only) / Senior1 |
| F17 lifecycle | retry_normal_exit, retry_abnormal_exit, refresh_coalesces, reconcile_nonactive, reconcile_terminal | Startup cleanup and complete event trace / Senior1 |
| F18 SSH | ssh_ipv6_bracketed, ssh_ipv6_unbracketed, ssh_missing_binary, ssh_shell_quote, ssh_remote_launch | Cancellation, forwarded secrets, binary/line framing / Senior1 |
| F19 CLI | cli_guardrail, cli_default_workflow | Explicit workflow path, logs root, startup error / Senior1 |
| F20 release gates | none | Offline reference suite, Rust locked gates, SQL/browser/four-target packaging (later gate); no live suite / QA+Operations |

## Scope of extraction and remaining gates

Each present case records a checked-in reference test anchor, synthetic input and expected assertion projection. Some expected fields compress multiple assertions into a portable semantic summary; they require Rust harness implementation and reviewer signoff before parity can be claimed. The assertion index contains source expressions, some multiline assertions are represented by their first line only, and it is not a substitute for normalized cases. No Rust runtime is implemented in this fixture set. Local/SSH cancellation and secret boundaries, full API response bodies, and accounting edge cases need additional cases; the table makes those explicit rather than treating one sample as completion. Live provider/model suites remain excluded. R01 remains Changes Required.
