# R01 provider-fixture coverage manifest

Reference revision: `be10a1b79df723d6d7612b5651c8522704dafb2e`. Cases are assertion-level transcriptions from checked-in offline tests, not runtime captures.

Represented: GitHub normalization, PR gating, malformed records, state paging, numeric refresh/dedup/404, native tool argument/status/failure handling; Jira ADF, inward Blocks, concrete enhanced-search request/cursor transcript, batch boundaries/order/scope omissions, blocker refresh, native tool status/failure; Asana complete normalized/native-ref fields and variants, concrete section-paging and refresh transcripts, native tool status/failure; GitLab complete normalized/native-ref fields and variants, paging/state mapping, empty reads, IID refresh, native tool status/failure; Linear blocker/assignee normalization, malformed refresh, 50-item batch refresh, environment aliases, bound-session snapshot, and native GraphQL success/error/input/transport contracts.

## Unrepresented contracts (explicit follow-up inventory)

- Linear cursor state pagination/termination and unassigned/assignee-mismatch variants beyond the selected blocker and ID-batch tests. The source has no `linear_adapter_test.exs`; Linear assertions are in `workspace_and_config_test.exs`, `core_test.exs`, and `dynamic_tool_test.exs`.
- Provider configuration validation and secret-environment declaration for all five adapters.
- Full raw-field preservation for Jira/Asana/GitLab and date/description edge cases beyond assertions represented here.
- Native-tool malformed argument matrix, non-JSON response serialization and every named transport/auth reason for GitHub/Jira/Asana/GitLab.
- GitLab request-path encoding and provider setting validation; GitHub/Jira/Asana HTTP request validation beyond the selected assertions.
- Cross-provider tracker binding/reload and dynamic-tool session-auth snapshot beyond Linear's selected GraphQL outcomes.
- All non-provider R01 contracts: config aliases/reload, prompts/Liquid, workspace hooks, app-server framing, token accounting, HTTP envelopes, scheduler retry/reconciliation, SSH cancellation/secret isolation.

This manifest is deliberately not an acceptance claim: the Rust comparison harness and exhaustive mapping remain outstanding.
