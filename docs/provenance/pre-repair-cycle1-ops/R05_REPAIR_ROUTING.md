# R05 repair cycle 1 — 2026-09-26

State: Changes Required. Senior technical findings, not an independent QA verdict. PM read the complete Senior2 review. Operations reports integrated code f64fa76ca47becb426defaf13d7925d1665b1cb7 and documentation HEAD 3d1575da917587c9795090f0dc8e51f4de1a990d on ops/symphosium-s0. Prior nine passing tests do not cover the reproduced failures.

Review: /Users/benblum/.openmausbot/task-workspaces/f348d16c-982a-4e00-83b7-b2a582504100/9a0808ee-9b00-4d7b-88c4-950d3b2553db/R05_FOUNDATION_CODE_REVIEW.md

| ID | Severity | Required correction | Developer | Review / QA |
|---|---|---|---|---|
| S2-R05-001 | High | Compare immutable resolved kind/org/project/assignee scope; reject changed effective scope with active claims, including environment changes | Senior1 | Senior2 / QA2 |
| S2-R05-002 | Medium | Retain and validate existing runtime policy, hooks, worker, observability, server, codex and per-state concurrency configuration | Senior1 | Senior2 / QA2 |
| S2-R05-003 | Medium | Restore reference default prompt body/fallback and contextual Liquid compatibility tests | Senior1 | Senior2 / QA2 |
| S2-R05-004 | Medium | Stable-order normalized label deduplication, preserving fail-closed blank semantics | Senior1 | Senior2 / QA2 |
| S2-R05-005 | Medium | Validate CLI ports, logs root, regular workflow file and path expansion; retain validated overrides in startup context | Senior1 | Senior2 / QA2 |

Repair cycle 1 is assigned; unsuccessful repair/retest cycles: zero. After two unsuccessful cycles for the same issue, escalate to CTO. No defect is closed by developer self-report.

Sequence: Senior1 isolated repair and evidence → Operations integrated checkpoint → Senior2 re-review and QA2 independent acceptance on exact integrated SHA. Operations preserves this review and routing now. R06/R07 integration remains dependency-gated on stable R05 contracts. No schedule advancement or release based on this return. Senior1 retains 20% review reserve; repair effort update is due with the patch rather than inventing a completion date.

R01/R02 S0 readiness remains accepted at cd2a7e98eaddfa988e093371ec1e227779a9a50f. RuntimeUnavailable outside CLI validation remains an intentional R05 boundary; scheduler dispatch belongs to R12. Toolchain evidence differs by environment: Operations reports pinned 1.97.1 passes; Senior2 used stable override. Repeat pinned integrated checks after repairs. No push retry without credential change; no release tag or paid/live provider/model calls.
