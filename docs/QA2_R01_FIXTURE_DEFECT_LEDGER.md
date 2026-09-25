# QA2 R01 fixture defect ledger

**Retested integration SHA:** `2472570251c646a0f1787fcf76d46279e300de56`  
**Reference:** `be10a1b79df723d6d7612b5651c8522704dafb2e`

| Defect ID | Severity | Status | Repair cycle | Independent retest evidence | Limitation |
|---|---|---|---:|---|---|
| QA2-R01-FX-001 | Medium | Closed — fixture review | 1 | `config_aliases` complete synthetic alias/map expectation and exact source fragments | No Rust config execution |
| QA2-R01-FX-002 | Medium | Closed — fixture attribution review | 1 | Reference completed-line behavior retained; pre-newline condition correctly LLD-only | No Rust framing execution |
| QA2-R01-FX-003 | Medium | Closed — fixture review | 1 | Asana/GitLab concrete normalized/native-ref fields and variants source-compared | No Rust normalization execution |
| QA2-R01-FX-004 | Medium | Closed — fixture review | 1 | Jira/Asana request/cursor/batch/order transcripts source-compared | No Rust provider execution |

Closed fixture-review rows are not product defects repaired in executable code and do not count as R02 or release acceptance. New relevant corpus changes require independent retest.
