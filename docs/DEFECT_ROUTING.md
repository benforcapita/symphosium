# R01 fixture correction routing

Baseline reference: be10a1b79df723d6d7612b5651c8522704dafb2e.
Current local integrated discovery checkpoint verified by PM: 2d5d6dce6627d3d6f373f5cd991cb2fd84c5de68, rust-integration.
Canonical checkout: /Users/benblum/.openmausbot/task-workspaces/cd435acf-e060-4630-82c2-2ae4213094bd/dba89769-72e3-4634-85e4-5ec2690e52af/symphony-stage0-consolidated.
QA authority: /Users/benblum/.openmausbot/task-workspaces/7b57fc4a-da28-4503-a744-8f65fb12bcd8/423a909a-4cd2-4bf9-9be5-5be0e2cedc0e/QA2_STAGE0_REVIEW.md, SHA256 05d8b1ef2b3ca2e7b0a907c12565924eaf6d8bf83730c43cecc836b6687aec9e.

| Defect | Severity | State | Developer | QA | Failed repair/retests |
|---|---|---|---|---|---|
| QA2-R01-FX-001 config alias/map fidelity | Medium | Changes Required | Senior 1 | QA2 | 0 |
| QA2-R01-FX-002 fragmented app-server line fidelity | Medium | Changes Required | Senior 1 | QA2 | 0 |
| QA2-R01-FX-003 Asana/GitLab normalized field fidelity | Medium | Changes Required | Mid 1 | QA2 | 0 |
| QA2-R01-FX-004 Jira/Asana paging/refresh transcript fidelity | Medium | Changes Required | Mid 1 | QA2 | 0 |

Follow exact case and reference anchors in QA review. Fix iterations return patches/artifact hashes and structural checks to PM; Operations integrates and checkpoints; QA2 independently retests that SHA. A structural pass is not a fidelity verdict. Two failed repairs/retests on the same defect escalate to CTO without relaxing gates.

Also close the declared discovery-map omissions by mapping required behaviors to actual reference scenarios, planned Rust tests and explicit later execution evidence. No mechanical requirement to normalize all 815 assertion starts. Senior 1 owns runtime/security/config/observability map; Mid 1 owns provider map. LLD-only behavior is labeled as such, never invented reference evidence.

R01/R02 remain Changes Required. R02 blocked by crates.io DNS/incomplete cache, not by fixture edits. R03 awaits CTO decisions. Four Medium fixture defects prevent acceptance; no application implementation verdict exists. Release/push remains blocked pending acceptance and verified fork destination. CTO technical escalation remains in parent task return because coordinate_bots rejected ancestor routing.

## Repair cycle 1 returned — integration and QA pending

PM read Senior1 repair notes and independently reran both portable validators successfully: 38 runtime cases and 40 provider cases. These checks do not close defects. All four remain Changes Required; first repair supplied, failed independent retests 0.

Operations inputs:

- Senior1 directory `/Users/benblum/.openmausbot/task-workspaces/68f56f75-eb0c-4f85-9c59-ee3f5c968b78/279d0ca2-388d-42ff-96ef-ae839cf89943/`: R01_FX_REPAIR_CYCLE1.patch (reported SHA256 56cc38b66512777e1d6eff04271e5a9440edc64f38660b40c2965231b542da2b), R01_FX_REPAIR_CYCLE1.md, R01_RUNTIME_GAP_MAP_v4.md, fixtures-v1/cases.json, check_cases.py, lld_only_tests.json. Apply patch only after reviewing paths/base; preserve extra gap-map/repair-note artifacts as needed.
- Mid1 directory `/Users/benblum/.openmausbot/task-workspaces/5160d311-3971-4930-b990-f54f9ff1dfc4/99779595-af6b-4586-895c-5bca0fc0036e/fixtures-r01-provider-v1/`: cases.json, check_cases.py, COVERAGE_MANIFEST.md. Import to docs/fixtures-provider-v1 without touching runtime corpus.
- This routing document and latest QA review must also be checkpointed.

FX-002 provenance distinction needs QA attention: Senior1 reports the reference test asserts successful handling of complete lines but not a separate pre-newline non-completion check. Stronger pre-newline behavior is explicitly represented as LLD-only instead of falsely attributing it to a reference assertion. QA2 must verify both attribution and the proposed test boundary.

Next sequence: Operations reviewed local integration/checkpoint → QA2 independent retest on resulting SHA → PM disposition. No repaired integrated SHA yet. Room handoff budget was exhausted on last Operations attempt; if it persists, parent CTO must resume this exact handoff through an available coordination context. Do not bypass Operations or QA ownership.
