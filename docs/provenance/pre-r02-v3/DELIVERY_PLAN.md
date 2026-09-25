# Symphosium Rust delivery plan

Planning baseline: 2026-09-25. PM owns this document. Technical authority: canonical docs/PROJECT_LLD.md revision 3, including normative sections 11–12. Original HLD retained as provenance; CEO owns its amendment. No scope reduction, paid spend or hiring.

## Provenance and present evidence

Source: /Users/benblum/.openmausbot/task-workspaces/d6f30c19-02d5-4618-a1fe-3ce7aec47fd1/5f84a53f-c70a-4bd5-8fb4-338abf6c691d/symphony-linear

Design directory: /Users/benblum/.openmausbot/task-workspaces/544e1ab9-982c-4f28-9639-867b565b5e01/fd3ce583-eb9d-4ba4-af14-d2500f06fb9f/symphony-linear-design/

PM read all three design documents and verified source HEAD be10a1b79df723d6d7612b5651c8522704dafb2e and untracked docs/PROJECT_HLD.md. No source changes or tests performed by PM. Integrated Rust revision: none. QA2 stage-0 verdict: Changes Required on v2; latest 38+33 fixture corpus review pending. Latest PM-verified discovery checkpoint: 0f33d80513223d90d87b3b8a572d21b0cd3cedfd. No remote publication verified.

## Estimates and capacity

Ranges below are provisional engineering person-days (8 effort hours), including ordinary review/rework; QA separately allocated in R19. Elapsed estimates assume one primary owner per ticket and exclude dependency wait. They are sizing assumptions, not measured bot throughput or calendar promises. Direct monetary spend is zero; internal compute/labor prices are unavailable, so currency cost cannot be calculated. Re-estimate after R01–R04 using actual effort, Rust competency evidence and declared availability. No completion date committed.

Senior 1 owns runtime/adapter ambiguity; Senior 2 owns security and reviews runtime. Each senior reserves 20% capacity for counterpart review, with at most one implementation ticket active. Mids own bounded domain/UI features after contracts stabilize; juniors own explicit fixtures/docs. QA starts alongside discovery. Owners must report Rust experience, availability, competing assignments and estimate corrections in first handoff. Unknown capacity blocks calendar commitment, not discovery. No demonstrated hiring gap.

## Backlog

All tickets priority P0 unless P1 shown. Complexity H/M/L. S0 discovery, S1 foundation, S2 boundaries, S3 UI/runtime, S4 acceptance. Owner abbreviations S/M/J/Q denote Senior/Mid/Junior/QA and number; Ops is Operations. Current states are recorded in STAGE0_REVIEW.md; initial assignments were R00–R04 and R19 planning. Ready means assigned for initiation, not confirmed execution.

| ID | Deliverable / acceptance criteria | Complexity | Effort days / elapsed working days | Dependencies | Developer; review | QA | Sprint |
|---|---|---|---|---|---|---|---|
| R00 | Isolated integration branch; preserve untracked HLD byte-for-byte; import design and PM plan; report paths, diff, local SHA and remote status without editing CEO worktree | L | 0.5–1 / 1–2 | none | Ops; PM | Q1 | S0 |
| R01 | SPEC 4–18/Appendix A and reference extensions mapped to Rust modules, sanitized fixtures and named tests; every required behavior traceable; differences explicit | H | 4–7 / elapsed blocked pending fixtures | none; checkpoint R00 | S1; S2 | Q2 | S0 |
| R02 | Rust dependency/capability spike proves Tokio/Axum/SQLx/template/YAML/Liquid/Markdown/auth choices, licenses, locked build and toolchain feasibility; four-target packaging risks and actual competence recorded | H | 2–5 / elapsed blocked by dependencies | none; checkpoint R00 | S1; S2 | Q2 | S0 |
| R03 | Adversarial scope/auth/lifecycle/isolation review; concrete threat tests and unresolved decisions sent to CTO | H | 2–3 / 3–4 | none | S2; CTO for decisions | Q1 | S0 |
| R04 | Sanitized fixture inventory and documented offline reference commands; identify toolchain/live-suite blockers without claiming passes | L | 1–2 / 2–3 | none; final mapping R01 | J1; S1 | Q2 | S0 |
| R05 | Cargo skeleton/config reload/CLI/prompt compatibility and external mode without DB; fmt/clippy/build and parity fixtures pass | H | 4–7 / 5–9 | R00–R03 | S1; S2 | Q2 | S1 |
| R06 | SQLx schema/migrations/seed and transactional identifiers/dependencies; concurrent creates/cycles, organization constraints and reversible disposable migration tests | M | 4–6 / 5–8 | R02,R03,R05 | M1; S1 | Q2 | S1 |
| R07 | Accounts/revocable sessions/roles/CSRF and PM-enabled observability protection; positive and negative auth tests | H | 4–7 / 5–9 | R03,R05,R06 | S2; S1 | Q1 | S1 |
| R08 | Domain CRUD/archive/comments/activity/filters/locking/idempotency and validation through one service; transactional/race/error tests | M | 5–8 / 6–10 | R06,R07 | M1; S2 | Q1 | S2 |
| R09 | All five external adapters and memory port; scope/paging/normalization/errors/native tools match versioned reference fixtures | H | 6–10 / 8–13 | R01,R04,R05 | S1; S2 | Q2 | S2 |
| R10 | Builtin adapter honors selected scope, assignee/blockers and batches; database failure never empty success | M | 2–4 / 3–5 | R08,R09 | M1; S1 | Q2 | S2 |
| R11 | API envelopes and issue-bound builtin_ticket tool; forged identity/reload/archive/idempotency/secret-boundary tests through real tool dispatch | H | 4–6 / 5–8 | R07,R08,R10 | S2; S1 | Q1/Q2 | S2 |
| R12 | Tokio scheduler dispatch/retry/reconciliation/accounting/approvals and shutdown match reference; stale completion and fault tests | H | 7–12 / 9–15 | R09,R10 | S1; S2 | Q2 | S3 |
| R13 | Local/SSH app-server framing/hooks/workspace containment/cancellation/secret isolation and startup/reload compatibility proven | H | 6–10 / 8–13 | R11,R12 | S1; S2 | Q2 | S3 |
| R14 | Project/list/detail forms, URL filters and stale-input retention; no-JS core flow and safe Markdown | M | 4–7 / 5–9 | R08,R11 | M1; S2 | Q1 | S3 |
| R15 | Board pagination, keyboard move, responsive controls and accessible errors; locked browser E2E command | M | 4–6 / 5–8 | R08,R14 | M2; S2 | Q1 | S3 |
| R16 | Bounded run projection/SSE/restart interruption and safe proof links; attempt completion distinct from ticket acceptance | M | 3–5 / 4–7 | R12,R13,R14 | M2; S1 | Q2 | S3 |
| R17 | P1 Explicit accessibility fixtures and setup docs under established patterns; keyboard/360px and no-JS instructions reproducible | L | 2–3 / 3–4 | R14,R15 | J2; M2 | Q1 | S3 |
| R18 | Clean install/deployment/backup restore/rollback, private worker isolation and four-platform artifact evidence; preserve reference packaging until accepted | M | 3–5 / 4–7 | R13,R16,R17 | Ops with J1 docs; S2 | Q1/Q2 | S4 |
| R19 | Independent A1–A10 integrated acceptance; security/browser/performance/fault/reference gates and defect ledger; test planning starts S0 | H | 12–22 QA days / 6–12 final gate days, environment dependent | final gate R05–R18 | developers repair own defects; PM routes | Q1/Q2 | S0–S4 |
| R20 | Release changelog/version/commit/tag and verified remote refs; CEO acceptance handoff with evidence and low issues | L | 0.5–1 / 1–2 | R19 Accepted; remote access | Ops; PM | Q1/Q2 signed SHA | S4 |

Revised sizing after QA estimates: 80–137 person-days including QA (QA1 6–12, QA2 6–10), plus 0.5–1 contingent R03 rework day. Initial estimate was 75–126; previous revision was 79–134. R01 increased to 4–7 and R02 to 2–5 after actual discovery. Review overlaps are capacity-limited, not unlimited parallelism. Rework beyond estimates triggers reforecast, never reduced gates.

## Dependency-aware Gantt (relative working-day envelope)

Illustrative earliest-start envelope only; D1 begins after capacity confirmation. Sequential senior work dominates. Review and discovery can extend these bands. Dependencies in backlog override graphical overlap.

| Stream | D1–10 | D11–25 | D26–40 | D41–55 | D56–70 | D71–85 |
|---|---|---|---|---|---|---|
| Operations | R00 | checkpoints | checkpoints | checkpoints | R18 | R20 after QA/remote |
| Senior 1 | R01/R02 | R05, review R06 | R09 | R12 | R13 | repairs/review |
| Senior 2 | R03 | R07 | R11 | runtime/UI review | security review | repairs |
| Mid 1 | capacity/contracts | R06 | R08/R10 | R14 | repairs | acceptance fixes |
| Mid 2 / Junior 2 | capacity | fixtures preparation | bounded UI preparation | R15/R17 | R16 | acceptance fixes |
| Junior 1 | R04 | docs updates | fixture support | docs updates | R18 docs | install fixes |
| QA 1/2 | R19 plans/R01–04 review | foundation tests | boundary acceptance | UI/runtime tests | failure/performance | integrated R19 |

No features integrate before stage-0 parity/spike review. UI prototypes may use fixtures but cannot establish backend acceptance. Senior 1 critical chain R01/R02 → R05 → R09 → R12 → R13 → R18/R19 → R20. Domain/security and UI chains converge at R19. Rebaseline immediately after discovery.

## Workflow, handoffs and gates

States: Backlog → Ready → In Progress → Review → QA → Accepted → Released; failed review/QA → Changes Required. A blocked ticket retains state plus reason/owner/unblock condition. Status changes require evidence, not assignment alone. Track defect ID, severity, affected SHA, reproduction, developer, QA, failed repair/retest count and low-defect disposition. After two unsuccessful repair/retest cycles on one defect request CTO diagnosis.

Each developer iteration returns repository/worktree/branch, ticket/sprint, base and result revision or patch, changed files, checks with outputs, blockers and next owner. Operations reviews diff and checkpoints locally, pushing only when access permits; developers must not edit another worktree. Operations integrates dependencies before QA; PM never fabricates QA verdicts. New code invalidates affected revision-specific evidence.

QA1 leads A1/A6 web auth/A7/A9 browser-install; QA2 leads A2–A5/A6 worker isolation/A8 and reference parity; both sign A10. Required commands: cargo fmt --all -- --check; cargo clippy --workspace --all-targets --all-features -- -D warnings; cargo test --workspace --all-features; cargo build --release --locked; unchanged Elixir make all with configured coverage. Discovery must name SQL, audit and browser commands and Rust coverage baseline. Missing tools are blockers. Live provider/model-cost tests remain unrun absent access/budget authorization; offline suites mandatory.

Acceptance uses one integrated SHA, all criteria and checks passing, zero critical/high/medium defects, tracked low defects, real 10k-ticket/20-project/10-client performance evidence (list/detail p95 <=500ms, propagation <=2s), local/SSH isolation tests and required packaging matrix evidence. Preserve source is not parity proof. No two runtimes dispatch the same scope concurrently.

Release handoff includes accepted integrated SHA/tickets, QA artifacts, open low issues and classification (initial Rust MVP; version chosen by Operations under repository convention). Operations alone owns release writes. Released requires verified remote commit/tag, so remote access blocks Released even if local acceptance succeeds. CEO owns final project acceptance; CTO resolves technical ambiguity. No approval is requested merely to adopt user-directed Rust.

## Current blockers / next owners

- R00 consolidation history verified through 0f33d80; 0bcac59 is in merged history. Mid1 provider corpus and latest PM documents await Operations checkpoint.
- R01 Changes Required: 38 runtime and 33 provider records pass structural checks; behavior coverage and assertion fidelity remain incomplete. Senior 1 owns runtime cases; bounded adapter extraction assigned to Mid 1.
- R02 Changes Required, blocked: installed direct Rust toolchain found, but missing cached ammonia and crates.io DNS prevent lockfile/build/license proof. Senior 1 owns spike; no repeated identical network retries requested.
- R03 Review: all eight technical decisions resolved in LLD3 section 11; independent QA mapping and Operations preservation pending.
- R04 Review: QA2 confirmed safe command classification; correct obsolete rustup/PATH claims using R02 v2 evidence.
- R19 In Progress: QA2 Changes Required is discovery-only; independent v2 review pending. No application acceptance.
- Origin verified by PM as https://github.com/benforcapita/symphosium.git; upstream is reference-only. Credentials/push unverified; Operations may attempt origin publication only after appropriate checkpoint review.
- Calendar capacity and successful Rust compilation remain unverified; no completion date committed. All R05–R18/R20 remain Backlog.


## Revision 3 continuation — current authority

Final canonical repository is the Source checkout above, not the earlier Operations discovery clone. PM verified origin/upstream and both untracked design files on this continuation. Operations first preserves HLD/LLD3 byte-for-byte on isolated integration branch, then imports reviewed discovery/repair artifacts without replacing LLD3 with old LLD2. Final committed Rust result must exist in canonical repository, with full SHA, clean integrated worktree, same-revision QA/build/lint/test/browser evidence. Historical handoff/DNS failures must be rechecked on needed actions.

R05/R12: immutable generation/claim serialization and drain failure blocks scope changes. R07/R16: transactional revocation locks, SSE pre-send authorization and <=15s revalidation. R08/R11: fresh attempt fence on every mutation/replay, empty-default explicit agent transition allowlist, stable scoped idempotency, documented lock order and <=3 retries. R13/R18: concrete worker filesystem/process isolation; fixed SSH helper/host verification, 5s heartbeat/30s lease, TERM10s/KILL5s and uncertain-exit fence. R07/R16: configured proxy CIDRs/hosts, cookie/CSRF rotation and rate limits. All details and exceptions remain those of LLD3 section11; QA tests T01–T13. Migration/rollback and previous-Rust requirement for builtin fallback are governed by section12.

Existing 80–137 person-day sizing and Gantt remain provisional pending actual bounded build; no new spend/hire. Four Medium fixtures remain open, first repairs supplied and structurally verified, zero failed independent retests. Operations imports then QA2 retests integrated SHA. R02 build retry now assigned; R05 remains dependency-gated.
