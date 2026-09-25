# R01 FX-001/FX-002 repair cycle 1

Project: Symphony Rust S0 R01. Reference baseline `be10a1b79df723d6d7612b5651c8522704dafb2e`; PM-reported integrated discovery base `2d5d6dce6627d3d6f373f5cd991cb2fd84c5de68` in `symphony-stage0-consolidated`. These edits are in Senior1's isolated task workspace, not integrated or QA retested.

## Changes

- FX-001: `fixtures-v1/cases.json` `config_aliases` now includes a synthetic API-key sentinel, exact top-level endpoint/API-key/project alias expectations, secret environment names, and complete normalized provider map including null assignee and extra data. `assertion_anchors` point to `workspace_and_config_test.exs:1202,1215–1218,1220,1224–1225`. The source test uses a distinct checked-in dummy literal; `synthetic_substitution` declares the fixture substitution.
- FX-002: `app_server_partial_lines` now represents four ordered emitted JSON lines: 1,100,000 padding bytes, thread ID, turn ID, and terminal event with process exit. Its anchors are `app_server_test.exs:1208–1209,1212,1215,1218,1245`. The reference asserts a successful run after complete lines; it does **not** separately assert non-completion before newline. `fixtures-v1/lld_only_tests.json` therefore states the stronger pre-newline boundary as an LLD-only Rust test, without inventing a reference assertion.
- `fixtures-v1/check_cases.py` verifies each new assertion-line fragment. `R01_RUNTIME_GAP_MAP_v4.md` maps 92 exact runtime/config/observability/security reference scenario anchors to planned Rust tests and later evidence; LLD-only categories are explicit. Provider repairs remain Mid1's assignment.

## Checks and status

`python3 fixtures-v1/check_cases.py --reference-root <baseline checkout>` passed 38 fixture anchors/schemas plus assertion anchors. `python3 -m json.tool fixtures-v1/lld_only_tests.json` passed. A script verified all 92 reference scenario lines in the map begin with `test`. These are structural checks only. No Elixir/Rust runtime test, build, network retry, QA retest, or integration commit occurred. FX-001/FX-002 remain open until Operations integrates and QA2 independently retests the integrated SHA. Repair/retest failure count remains zero before that retest; do not self-close them.
