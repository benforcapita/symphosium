# Publication copy note

This copy removes machine-specific filesystem paths. Source SHA-256: `c61aa6e52b2e54965c3d5e1d83014a6718e3f9a1ccb68ca3d857119f24b276ea`. Prior delivery plans remain preserved.

# Symphosium parallel delivery allocation — 2026-09-26
PM-owned addendum to DELIVERY_PLAN.md. Canonical repository: canonical project repository.
HLD and LLD3 remain authoritative. Existing files/worktrees must be preserved. This addendum supersedes stale scheduling status, not technical requirements.

## Evidence and correction
PM read QA2_R05_FOUNDATION_ACCEPTANCE.md: R05 independently Accepted at 23b3f7f1a14eca8ed27b3812d9db10437149fb98; all five defects closed. Do not repeat R05 repairs. New affected code requires fresh QA.
Local linked worktrees inspected: ops/symphosium-s0 at 67a9a45; ops/r06-database-wip at 6ef44a1. Earlier Operations publication was reported; PM remote verification for this allocation failed DNS resolving github.com. These are local checkpoints until Operations re-verifies the remote refs.
R09 remains Changes Required: two High and one Medium security findings, repair cycle 1 assigned; QA verdict pending.

## Immediate allocation
All rows are Ready/assigned until owner execution evidence; inherited R09 Changes Required, R06 Review and R19 In Progress remain unchanged. Branches are requested targets, not creation claims; preserve existing active owner branch where applicable and report exact mapping.
Priority P0 except documentation/packaging preparation P1. Estimates below are bounded first-increment effort hours / elapsed hours after start; exclude queue time. Dollar cost unknown; no paid commitment. QA acceptance is independent.

| Ticket | Owner | Branch target | Concrete first increment / acceptance | Dependencies and merge gate | QA | Complexity | Effort / elapsed |
|---|---|---|---|---|---|---|---|
| R09-FIX1 | Senior1 | dev/r09-security-fix1 (retain active repair branch if any) | Close S2-R09-001/002/003 with adversarial URL/redirect/repository tests and sanitized failures | accepted R05; Senior2 review, QA2 exact SHA | QA2 | H | 2–6 / 3–8 |
| R06-REV1; R09-REV1 | Senior2 | review/r06-r09-contracts | Rolling review R06 checkpoint and R09 repair; reproducible findings and exact regression fixtures | R06 checkpoint available; R09 review follows repair, never self-accept | QA2 | H | 2–4 / 3–6 |
| R06; R08-PREP1 | Mid1 | ops/r06-database-wip; dev/r08-domain-prep | Preserve R06, fix review findings; prepare domain DTO/validation tests against documented stable boundaries | R08 production merge blocked R06 + R07 acceptance; avoid provisional auth implementation | QA1/QA2 | M | 3–6 / 4–8 |
| R14-PREP1; R15-PREP1 | Mid2 | dev/r14-r15-ui-scaffold | Fixture-driven list/detail/board and API shape scaffolding, loading/errors/stale-input/no-JS/keyboard cases | Production binding blocked R08/R11, board integration R14; fixtures cannot define API authority | QA1 | M | 3–6 / 4–8 |
| R04-FX2 | Junior1 | test/r04-deterministic-fixtures | Deterministic config/provider fixtures with stable expected assertions and reproducible validation | accepted R05, existing corpus; no duplicate fixture audit | QA2 | L | 1–3 / 2–4 |
| R18-PREP1; R17-PREP1 | Junior2 | docs/r18-packaging-ci | Current lockfile license inventory, installation/CI fixtures and packaging gap matrix; distinguish proven targets | Real release packaging blocked R13/R16/R17; no fabricated platform passes | QA1 | L | 2–4 / 3–5 |
| R19-Q2 | QA2 | qa/r06-r09-acceptance | Preserve R05 exact-SHA acceptance, prepare/run independent R06 tests and R09 security retest when checkpoint ready | Exact Operations checkpoint + senior review for acceptance | QA2 | H | 3–6 / 4–8 |
| R19-Q1 | QA1 | qa/r19-security-integration | Executable auth/security/integration/compatibility suites mapped to LLD T01–T13; explicit unavailable boundaries | Test preparation now; passing integrated verdict requires implementation | QA1 | H | 3–6 / 4–8 |
| R07-CONTRACT1 | CTO | design/r07-domain-contracts | Resolve only concrete R06/R07/R08/API interface ambiguities and lock order; decide safe next R07 implementation split | HLD/LLD3; technical decision owner CTO | QA1 | H | 1–3 / 2–4 |
| R00-OPS-PAR1 | Operations | ops/symphosium-s0 plus each owner branch | Checkpoint/push every supplied increment; return branch, full SHA, remote ref and URL; preserve canonical docs | No unaccepted production merge or release; avoid clobbering active worktrees | QA1/QA2 | M | 2–4 / ongoing |
| PM-PAR1 | PM | docs/pm-parallel-delivery | Allocation, dependency map, merge queue, exact evidence ledger; import this document through Operations | Owner handoffs update statuses; no PM-authored QA passes | QA1 | L | 1–2 / ongoing |

CEO retains scope/budget/final acceptance. Hiring Bot remains dormant: no concrete expertise gap or economic justification; hiring is prohibited by current no-spend direction. All delivery roles have useful streams.

## Dependency-aware Gantt and merge queue
Relative work windows, not promised calendar dates:
- W0 (now): all preparation/review/test/docs streams above start concurrently; Operations checkpoints each ready increment.
- W1: R06 review -> Operations repaired checkpoint if needed -> QA2 accepted database; concurrently R09 repairs -> Senior2 -> Operations -> QA2.
- W2: CTO-resolved R07 implementation by Senior2 when rolling review capacity permits (reserve 20% senior review capacity), Senior1 counterpart review, QA1. Mid1 domain implementation proceeds only against accepted contracts and merges after R06/R07.
- W3: R08 -> R10 (+ accepted R09) -> R11; R14 follows R08/R11, then R15. Fixture UI branches can progress throughout W0–W2.
- W4: R12 after R09/R10 -> R13 after R11/R12; R16 after R12/R13/R14; R17 after R14/R15.
- W5: R18 -> integrated R19 -> R20 release -> CEO acceptance.
Documentation/test-only commits may checkpoint independently; they confer no production acceptance. Operations serializes integration. New code invalidates affected test evidence. Two failed repair/retest cycles for one issue trigger CTO diagnosis.

## Checkpoint ledger
- R05 accepted source: https://github.com/benforcapita/symphosium/commit/23b3f7f1a14eca8ed27b3812d9db10437149fb98 (QA report read).
- Integration local head: 67a9a45, branch https://github.com/benforcapita/symphosium/tree/ops/symphosium-s0.
- R06 local head: 6ef44a1, branch https://github.com/benforcapita/symphosium/tree/ops/r06-database-wip.
- New stream first checkpoint SHAs/URLs: pending Operations; do not invent.
- Release gate: all HLD/LLD acceptance and required checks on integrated revision, zero Critical/High/Medium, tracked lows; Operations verified remote release commit/tag; CEO final acceptance.
