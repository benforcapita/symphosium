# QA2 independent R01 fixture repair-cycle 1 retest

**Project/repository:** Symphosium, canonical-linked integration  
**Revision under review:** `2472570251c646a0f1787fcf76d46279e300de56`  
**Reference baseline:** `be10a1b79df723d6d7612b5651c8522704dafb2e`  
**Authority:** `docs/PROJECT_LLD.md` revision 3, especially sections 11–12.  
**Scope:** static fixture/traceability retest. This is not a Rust build, runtime parity result, R02 disposition, product acceptance, or release sign-off.

## Exact evidence and checks

- `git -C symphosium-s0 rev-parse HEAD` returned the reviewed SHA above; `git status --short` was empty.
- Runtime structural validator: **38** fixture anchors/schemas passed.
- Provider structural validator: **40** fixture anchors/schemas passed.
- `python3 -m json.tool docs/fixtures-v1/lld_only_tests.json` passed.
- No formatting, compilation, dependency resolution, live-provider/model, Elixir, or Rust runtime test was run.

| Artifact | SHA-256 |
|---|---|
| `docs/fixtures-v1/cases.json` | `eab09daf61cd6acf8567f7c248ba4557439e7b2bd3ed12838b2465de8adda771` |
| `docs/fixtures-v1/check_cases.py` | `40fdeeb7f4066799364b183ebeb22cda7d7a5ac15086434633acb46a8ed38171` |
| `docs/fixtures-v1/lld_only_tests.json` | `f62d0bacce5464919a0e02be735d835c7fe5a796c4e75d9732446d7ac937647b` |
| `docs/fixtures-provider-v1/cases.json` | `881cf67025a418e306cc724c67d77b08d67ad09ccd72a5d4ecf1a18bcb29a6c9` |
| `docs/fixtures-provider-v1/check_cases.py` | `6e057d362639b66f677e5d9c6be5dafc08d748b741efc12a6c33668ffb41acbc` |
| `docs/R01_RUNTIME_GAP_MAP_v4.md` | `d0f79fbaa5d2b45410e9dcdaabde13cdad5bc1d581fb735727dd8f522e90fef6` |
| `docs/PROJECT_LLD.md` | `4411658ffc280eedd198f13dc2ca2801d2492aceb52f07c05fc458fa59ff96f0` |

Structural validation is supporting evidence only; it does not independently prove fixture fidelity or close a defect by itself.

## Independent defect retest

| Defect | Severity | Retest evidence | Verdict |
|---|---|---|---|
| QA2-R01-FX-001 | Medium | `config_aliases` now contains a declared synthetic API-key sentinel, expected top-level endpoint/API-key/project aliases, secret-environment name, complete provider map including `assignee: null`, and exact source fragments at `workspace_and_config_test.exs:1202,1215–1218,1220,1224–1225`. The substitution from reference dummy literal is declared rather than silently copied. | **Closed at fixture-review level.** Static sanitization is acceptable: the sentinel is not a credential value. Runtime config compatibility remains untested. |
| QA2-R01-FX-002 | Medium | `app_server_partial_lines` preserves the 1,100,000-byte completed JSON line, `thread-91`, `turn-91`, terminal event and successful-run anchors (`app_server_test.exs:1208–1218,1245`). Direct reference review confirms the producer uses one complete `printf` line and asserts post-line successful run; it has no separate pre-newline callback/non-completion assertion. `pre_newline_no_completion` is separately classified in `lld_only_tests.json` as an LLD strengthening, not reference parity. | **Closed at fixture-review/attribution level.** The stronger boundary still requires a Rust implementation test; it is not falsely attributed to Elixir. |
| QA2-R01-FX-003 | Medium | `asana_normalize_fields` and `gitlab_normalize_fields` now include concrete synthetic input and expected native-ref shapes plus ID, identifier, title, description, state, URL, assignee, labels, blockers, dispatchability and timestamps. Variant records cover source terminal/fallback/blank-title paths. Source anchors match `asana_adapter_test.exs:101` and `gitlab_adapter_test.exs:116` assertion groups. | **Closed at fixture-review level.** Rust type/DateTime conversion and actual normalization are later executable parity checks. |
| QA2-R01-FX-004 | Medium | Jira cases now state enhanced-search method/path/JQL/fields/cursor/returned and omitted IDs, plus bulk-fetch 100+1 grouping/order/scope outcomes. Asana cases now state section page path/query/offset and refresh request paths/order/404/out-of-project omissions. Manual comparison matched source scenarios at `jira_adapter_test.exs:247,317` and `asana_adapter_test.exs:145,190`. | **Closed at fixture-review level.** Provider validator checks nonempty listed assertion lines, so the manual source comparison—not its structural pass—is the closure evidence. |

## Remaining Stage-0 traceability gaps

R01 is **not accepted**. `R01_RUNTIME_GAP_MAP_v4.md`, runtime `COMPLETENESS.md`, and provider `COVERAGE_MANIFEST.md` still list required planned-fixture/map outcomes: config defaults/front matter/policy rules; prompt nested/error paths; workspace collision/timeout boundaries; complete app-server approval/MCP/parser/timeout/tool-failure paths; observability response/accounting edge cases; SSH cancellation/forwarded-secret isolation; Linear cursor/refresh/assignee coverage; provider configuration/secret-environment validation; raw-field/date/description and malformed/non-JSON native-tool paths; request validation; and cross-provider session binding/reload.

LLD3 section 11 adds Rust-hardening behavior that must remain explicitly labeled LLD-only rather than reference parity, including scope/reload claim fencing, authorization/revocation transaction boundaries, attempt-fence/idempotency behavior, production worker identity/secret isolation, fixed SSH helper lifecycle, and HTTP/SSE policies. The current LLD-only JSON covers only three examples; the gap map or a successor must provide a categorized planned-test mapping for the other applicable normative decisions.

The stage-0 boundary permits mapped, reviewed reference and LLD-only planned tests; it does not require mechanically porting the 815 assertion-start index. Full behavioral parity remains a later integrated Rust-runtime gate.

## Unchanged blockers and conclusion

R02 remains independently blocked: no resolved lockfile, successful bounded Rust build, complete dependency/license audit, or verified network/cache change. The four fixture defects are closed only for this exact documentation SHA; subsequent relevant edits invalidate this retest. No critical/high/medium implementation defect verdict and no product/release acceptance is issued.
