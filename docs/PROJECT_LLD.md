# Symphosium — consolidated Rust LLD

Status: CTO technical design ready for PM decomposition, Rust revision 3, 2026-09-25. The user explicitly requested Rust after the initial Elixir design; this revision supersedes that implementation choice. Implementation and QA remain pending.
Basis: adjacent PROJECT_HLD.md, copied unchanged from CEO checkout; upstream be10a1b79df723d6d7612b5651c8522704dafb2e.
Source checkout: /Users/benblum/.openmausbot/task-workspaces/d6f30c19-02d5-4618-a1fe-3ce7aec47fd1/5f84a53f-c70a-4bd5-8fb4-338abf6c691d/symphony-linear.
Canonical delivery destination is this repository, origin https://github.com/benforcapita/symphosium.git; upstream https://github.com/openai/symphony.git. Operations must checkpoint this design and integrate the finished Rust implementation into the canonical repository. Development uses isolated branches/worktrees. An artifact in another workspace is not final integration evidence.

## 1. Requirements and decisions

Deliver the HLD MVP: one configured organization, multiple teams/projects, persistent tickets, workflow states, users, assignments, labels, dependencies, comments/activity, list/detail/board views, filters, keyboard navigation, run visibility, scoped agent mutations, and reproducible deployment. Preserve all five external adapters and memory test adapter. No SaaS, signup, SSO, billing, paid service, distributed scheduler, or temporary hiring.

Implement the application in Rust under a new rust/ directory, preserving elixir/ as an untouched reference and supported fallback. Use a Tokio runtime, Axum HTTP routing, SQLx with PostgreSQL, Serde typed transport/config models, tracing structured logs, Reqwest provider clients, and Rust server-rendered templates with autoescaping. Use small first-party JavaScript modules for board interactions and server-sent-event refresh; core forms and navigation must work without JavaScript. No separate frontend framework is required. Select maintained Markdown, sanitizer, password-hashing, session and YAML/template libraries during the dependency spike, pin Cargo.lock and rust-toolchain.toml, and verify licenses/build support. These are design choices, not assertions of current dependency versions.

Use one locally hosted PostgreSQL database with a documented container option. SQL transactions provide identifier allocation and dependency integrity. External-only operation continues without a configured database. Database-backed features are explicitly enabled; built-in adapter selection requires them. Preserve the language-agnostic SPEC.md contract and existing implementation extensions through compatibility fixtures. This is a runtime port as well as a product extension: do not claim parity merely because Elixir remains in the repository.
Keep one configured tracker scope and one orchestrator per deployment, as upstream does. UI can manage multiple projects, but only tracker.provider.project_id is dispatched. Switching tracker kind/project while claims exist is rejected with an actionable config error; ordinary reload semantics remain unchanged. Scope-changing restart requires operators to drain active work first. Do not introduce automatic multi-project dispatch.

## 2. Components and ownership

Use one Cargo package initially, with modules and a thin binary; split crates only for a demonstrated boundary.

* config/workflow: immutable validated configuration snapshots, front-matter parsing, environment references, prompt rendering compatible with existing Liquid-style templates and last-known-good reload.
* tracker: async Tracker interface with normalized Issue, read methods and optional agent tool methods; implementations builtin, linear, github, jira, asana, gitlab and an in-memory test implementation. Bind adapter/config once per agent session.
* orchestrator: one Tokio task owns claims, running jobs, retries, blocked entries and counters. Typed messages and bounded channels serialize state changes; async I/O and runner tasks report tagged results. Attempt generation IDs discard stale completions. No shared mutable scheduler map accessible to HTTP handlers.
* workspace: canonical path containment, safe identifiers, symlink/path traversal checks and hook lifecycle. Never use source checkout as agent cwd.
* agent: local subprocess and SSH transport, app-server JSON protocol, bounded incremental framing, timeouts, approvals and issue-bound dynamic-tool dispatch. Explicit child cleanup on cancellation/shutdown; propagate process failures and preserve retry policy.
* db: SQLx pool, versioned migrations, explicit transactions and constraints. Never migrate implicitly at boot.
* project_management: authoritative CRUD, authorization, validation, transactional activity, dependencies and optimistic concurrency. All transports call this service; no handler or tool writes SQL directly.
* accounts: operator-provisioned accounts, password verification, revocable sessions and roles; no signup or email dependency.
* web: Axum routes, HTML templates/forms, JSON API and authenticated SSE. /pm holds product routes; retain / and /api/v1 observability contracts. Authentication middleware plus authorization at every service operation.
* run_projection: persist bounded attempt summaries and publish identifier/version notifications, without scheduling jobs.

Rust ownership replaces OTP supervision, not its recovery semantics. Define failure policy explicitly: a runner panic becomes an attempt failure and retry; critical orchestrator failure terminates the service for supervisor restart; database outage fails reads/mutations without manufacturing empty tracker results. Bound channels and response bodies, set transport deadlines, cancel and join task-owned work during shutdown, and verify startup/reload/restart together.

Follow root SPEC.md, license/notices, PR template and logging field conventions. elixir/AGENTS.md governs changes under elixir/; keep its implementation intact. Add rust/AGENTS.md with formatting/lint/test/security conventions and rust/README.md plus a Rust WORKFLOW example. Do not introduce Elixir-specific @spec or Mix requirements into Rust source. All public interfaces have Rust types and rustdoc describing invariants/error behavior.

## 3. Data model and invariants

All primary IDs are UUIDs; timestamps are UTC microseconds. Every organization-owned record carries organization_id. Enforce organization consistency with composite foreign keys where possible and transactional context checks otherwise.

| Entity | Required fields and constraints |
| --- | --- |
| organizations | id, immutable unique slug, name; deployment config selects exactly one |
| users | org, normalized unique email, display_name, password_hash, role admin/member, disabled_at; disabled users cannot authenticate |
| sessions | user_id, hash of random opaque token, expires_at, revoked_at; never persist raw token |
| teams | org, name, immutable uppercase key unique within org, next_issue_number >= 1 |
| projects | org, team_id, name, slug unique within org, archived_at |
| workflow_states | org, team_id, name unique within team, category backlog/active/terminal, position; referenced states cannot be deleted |
| tickets | org, team_id, project_id, number, immutable identifier, title, description, priority nullable or 1..4 (1 highest), state_id, assignee_id nullable, lock_version, archived_at, timestamps |
| labels / ticket_labels | org, normalized label name unique within org, display name, color; unique ticket/label join |
| dependencies | org, ticket_id, blocker_id; unique pair, no self edge, same configured organization; MVP requires same project |
| comments | org, ticket_id, author user or agent-run reference, Markdown body, timestamps, lock_version |
| activity | org, ticket_id, actor reference, operation, sanitized changed-field metadata, timestamp; append only |
| mutation_keys | org, actor scope, key, request hash, sanitized result; unique scope/key; retain 7 days |
| run_attempts | org, ticket_id, attempt UUID, session reference, lifecycle status, timestamps, bounded reason and approved proof links; unique attempt |

Allocate team ticket number by row lock and increment in the ticket creation transaction. Identifier TEAM-123 is unique organization-wide and never reused. Team key is immutable; moves between same-team projects preserve identifier; cross-team moves are rejected in MVP. Stable URL uses ticket UUID under /pm/tickets/:id. Unique constraints cover org/identifier and team/number. Concurrent creation across projects must prove collision freedom.

Archive projects/tickets instead of destructive deletion. An archived active ticket becomes non-dispatchable and is reconciled using the existing non-active behavior; archive does not falsely mark it terminal. Do not delete comments/activity as part of archive. Explicit comment editing/deletion is author-or-admin only, leaves an activity tombstone, and is unavailable to agent tools in MVP.

Seed Backlog, Todo, In Progress, Done and Cancelled once per team using idempotent setup. Workflow config explicitly selects active and terminal state names. Validate categories against configured names; prohibit ambiguous duplicate state names in the selected team and prohibit state renames that invalidate the selected workflow. Custom state management is admin-only. All built-in active candidates with unfinished blockers are non-dispatchable, including newly blocked running tickets; generic reconciliation owns stopping behavior. Dependencies are acyclic: serialize edge changes under a project row lock and run a recursive reachability check in the same transaction. Closed blockers cease blocking; missing/inaccessible blockers fail closed.

## 4. Adapter and configuration contract

Add tracker.kind: builtin, tracker.provider.project_id (UUID), optional assignee UUID. Preserve active_states, terminal_states, required_labels and all existing config aliases. Host config adds feature enablement, database URL reference, public base URL, session signing secret reference and organization UUID; secret values must not be literal in repository workflows. Built-in startup requires reachable migrated database, valid project/organization and explicit active/terminal state policy. External mode never requires database or new auth settings when PM is disabled.

The async fetch_issues_by_states and fetch_issues_by_ids methods return Result<Vec<Issue>, TrackerError>, preserving the semantic success/error contract of the reference callbacks. Empty inputs return empty results. Reads always filter selected organization/project and exclude archived records; IDs outside scope are omitted. Scan all requested candidates in bounded internal batches rather than silently truncating scheduler reads. Database failures return tracker_transport-class errors, never successful empty responses. Invalid config maps to tracker_config, malformed rows to tracker_payload; include safe detail.

Issue.id = UUID; identifier = immutable TEAM-number; native_ref = nonsecret project/ticket UUID object; priority = nullable 1..4; state = exact state name; description = Markdown; branch_name = nil initially; URL derives from validated configured origin, never request Host. Labels are normalized/deduplicated; blocked_by uses the reference adapter blocker shape and current blocker states. Read and test the existing client normalization fixtures before implementing that mapping. dispatchable requires configured assignee match and no unfinished blocker. Scheduler continues to own labels, claims, retry, concurrency and terminal cleanup.

Use a stable configuration generation for each adapter read. Session-bound tools retain their original scope across reload; current ticket authorization is rechecked on every mutation. Port each external adapter faithfully behind the Rust interface; leave its Elixir implementation intact. Compatibility fixtures must cover provider scope, paging, IDs, labels, blockers, error envelopes and advertised native tools, including their existing credential-limited raw-tool access.

## 5. API and agent tool contracts

Authenticated /pm/api/v1 exposes projects, tickets, states, users (safe fields only), labels, ticket dependencies and comments. GET collection/item, POST collection, PATCH item; DELETE project/ticket means archive; relationship DELETE removes the edge; state/user administration and provisioning are admin-only. Collection filters: project, state, assignee, label, priority, case-insensitive title/identifier search. Cursor pagination defaults 50, max 100; deterministic updated_at/id ordering. UI uses the same filters in its URL.

JSON success: {data: resource, meta: {next_cursor}} where relevant. Errors: {error: {code, message, fields}, request_id}. Statuses: 400 malformed, 401 unauthenticated, 403 forbidden operation, 404 missing/out-of-scope resource, 409 stale lock/cycle/idempotency conflict, 422 validation, 429 throttled, 503 transient database failure. No stack traces or SQL details. PATCH includes lock_version; require it and reject stale writes. Creation/comment POST accepts Idempotency-Key; same key/body replays original result; different body returns 409. Commit activity and mutation result atomically with the resource change.

Agent dynamic tool name builtin_ticket, operations get_current, add_comment, transition. Inputs: operation, body for comment, state_id and lock_version for transition, idempotency_key for mutations. Reject unknown fields, arbitrary ticket/project IDs, SQL, URLs and unsupported operations. Ticket identity comes solely from trusted issue option in the Rust run_turn host context (matching the reference AppServer.run_turn boundary) and bound tracker settings. Re-fetch ticket to verify it still belongs to bound project/org and is not archived. No actor/role accepted from model input. Tool result preserves existing success/output/contentItems envelope with sanitized JSON. In-process dispatch through this application boundary avoids giving an API token or database credential to the agent. HTTP API uses human authenticated sessions in MVP; independent external machine-token access is deferred.

Title 1..240 characters; description <=64 KiB UTF-8; comment 1..16 KiB; request <=128 KiB; labels <=50 per ticket; dependency count <=100. Validate UUIDs, enum values, referenced records, normalized unique names and ownership. All API/UI writes enforce identical context authorization, including form/API submissions and SSE reconnects.

## 6. Security and deployment boundary

Trusted single-organization deployment; users are admin/member and may access all projects in that organization. This is not per-project confidentiality. Agents are less privileged: only current ticket read/comment/transition. Admin provisions users by local operator task, never command-line plaintext passwords; documented interactive input. Password hashing uses a maintained compatible library selected and locked during stage 1. Use secure HttpOnly SameSite session cookies, CSRF on cookie-authenticated mutations, origin checks for streaming connections, login rate limits and revocable expiring sessions. Admin-only configuration and user/state management; members manage ordinary resources/comments.

PM-enabled deployment authenticates existing observability pages and APIs too because they expose runtime data and refresh writes; external-only mode preserves existing trusted-network behavior. Document this intentional opt-in difference and test it. Reverse proxy terminates TLS, accepts only configured hosts, and does not trust client identity headers. Bind application/database to private interfaces; no public database port. Explicit loopback HTTP development mode only. No CORS wildcard.

Render Markdown with raw HTML disabled and a sanitizer allowlist. Reject javascript/data URLs and remote image embedding; links allow http/https with safe rel attributes. Never fetch proof URLs server-side. No raw logs, prompts, environment dumps, tokens or arbitrary filesystem links in ticket views/API. Use CSP compatible with the first-party scripts and original visual identity.

Strip database, session, account-bootstrap and tracker credential variables from child environments for every adapter; cover SSH command forwarding as well. Keep secret files outside source/workspaces with restrictive permissions. Scrubbing environment alone is insufficient: production agent workers run with a separate OS/container identity and filesystem boundary denying service secret files/database socket access. Trusted local development must document its weaker isolation. Do not claim sandbox resistance to a hostile host administrator. Hooks and workspace bootstrap receive the same secret-boundary review. Log only safe issue_id, issue_identifier, session_id, operation/outcome and request_id.

Deployment path: one application instance plus private PostgreSQL, migrations as an explicit operator release task, idempotent seed and admin bootstrap, TLS proxy, health/readiness checks, volume backups and restore rehearsal. Never put secrets in images. Database failure makes PM readiness fail and tracker reads fail closed while existing reconciliation avoids treating an outage as an empty tracker. Rollback: backup first; reversible schema migration down tested on disposable data; destructive rollback restores backup with a compatible binary. Never promise lossless destructive down migrations.

## 7. UI and run projection

/pm shows project list. Project page toggles list and board, preserving filters. Ticket detail supports title/description, assignment, priority, state, labels, blockers, comments/activity and run status. Board has one column per workflow state, bounded pagination and explicit load-more. Drag/drop is optional convenience; keyboard-accessible Move to state menu is required. Pending writes have visible progress and errors; a stale edit retains user input and offers reload. Responsive at 360px and desktop; semantic controls, visible focus, accessible labels, contrast, keyboard operation and no drag-only action.

Ticket workflow state and attempt status remain separate. Projection statuses queued/running/blocked/retrying/completed/failed/interrupted; queued means eligible waiting, not a persisted scheduler claim. Blocked distinguishes dependency from agent-input reasons. Completed means attempt completion, never proof of ticket acceptance or merged PR. Persist bounded attempt summaries from existing lifecycle events, idempotent by attempt/event identity. Startup marks unresolved previous-process attempts interrupted, then reconciles current snapshot. Do not restore claimed/blocked scheduler state from this table. SSE updates carry identifiers/version only; clients re-read authorized data. Lost event recovery uses periodic runtime snapshot reconciliation; show last-observed time and unknown rather than inventing history after crashes.

Indexes: project/state/updated_at/id, assignee, label joins, dependency ends, org/identifier, ticket activity timestamp and ticket/run timestamp. Avoid N+1 blocker/label loads. QA measures on a declared local baseline with 10,000 tickets, 20 projects and 10 concurrent UI clients: list/detail p95 <=500ms server-side, visible mutation propagation <=2s, no unbounded list/board responses. These are acceptance targets, not measured claims. PM may negotiate sizing only via CTO/CEO scope review.

## 8. Implementation sequence and staffing

PM owns tickets, dates, Gantt, actual assignments and capacity confirmation. Proposed dependency sequence and role fit below are planning input, not direct developer assignments.

1. Foundation: Senior 1 leads the Rust runtime/adapter compatibility spike, then SQLx schema/config and migration smoke; Senior 2 adversarially reviews auth, scope and lifecycle design before large implementation. Junior 1 supports fixtures/setup docs. Deliver schema constraints and identifier/dependency concurrency tests first.
2. Boundaries: Senior 1 ports all external adapters and builds the built-in adapter; Senior 2 builds Accounts/context/API/tool scope. Mid 1/2 support domain operations and integration fixtures. Dependency: foundation. Gate on issue-bound authorization, secret isolation, adapter contract and external regressions before UI integration.
3. User workflow: Mid 1 list/detail, Mid 2 board/filter/keyboard interactions; Junior 2 assists components and accessibility fixtures. Dependency: stable context contracts; UI prototypes can proceed earlier against fixtures. QA 1 independently tests workflow persistence and API security.
4. Runtime: Senior 1 completes the Rust scheduler, local/SSH runner parity, lifecycle projection and reconciliation, Senior 2 reviews startup/reload/failure paths; Mid 2 integrates run views. QA 2 owns scheduler regressions and fault/restart/SSH secret tests. Dependency: adapter and domain.
5. Hardening/release: QA 1/2 independent integrated verdict, accessibility/performance/E2E; juniors finalize clean-install docs with Operations; Operations owns checkpoints, deployment/backup rehearsal and release writes. Dependency: all acceptance evidence.

No demonstrated expertise gap: work fits the existing senior backend/architecture, mid UI/domain, junior docs/fixtures, two independent QA and Operations roles. Their Rust proficiency and available hours have not been verified; role labels alone are not proficiency evidence. PM must collect capacity and have early foundation/prototype reviews establish competence. Planning assumption: two seniors can cover auth and asynchronous runtime work concurrently, two mids cover UI/domain, QA starts from stage 1. Do not assert a calendar or comparative cost without measured throughput. Zero monetary budget rules out paid dependencies/services. Hiring Bot is not needed now. Escalate a specific reproducible expertise block or measured schedule/cost comparison to CTO/CEO before considering temporary staffing; never hire from mere uncertainty.

## 9. Acceptance and verification matrix

| ID | Required observable evidence | Independent owner |
| --- | --- | --- |
| A1 | Clean migration/seed; create project/ticket, edit and move from list/board/detail; restart preserves all values | QA 1 |
| A2 | Concurrent cross-project creates have unique stable identifiers; stale write conflicts; transactional cyclic edge insertion rejected | QA 2 |
| A3 | Built-in candidates dispatch; blockers/assignees/labels/concurrency honored; mid-run changes reconcile; terminal stops and safe cleanup match upstream | QA 2 |
| A4 | Real application dynamic-tool path comments/transitions only current ticket; forged identity, moved/archived ticket, reload and duplicate mutation rejected or replayed correctly | QA 2 |
| A5 | Linear/GitHub/Jira/Asana/GitLab and memory contract/regression tests green; no DB required in external mode; existing routes/config aliases preserved | QA 2 |
| A6 | Unauthenticated requests/form events blocked, cross-org references hidden, CSRF rejected, Markdown XSS/unsafe URL payloads inert; local/SSH child cannot read service secrets | QA 1/2 |
| A7 | Keyboard-only full lifecycle; mobile/desktop browser checks; list/board pagination and measured performance meet section 7 | QA 1 |
| A8 | Attempt/issue status distinction, blocked/retry and restart interrupted state shown; no false completed ticket; safe proof metadata | QA 2 |
| A9 | Fresh install from docs, production build/start/migration, backup/restore and rollback rehearsal with disposable DB; feature-disabled binary smoke | QA + Operations |
| A10 | Same integrated SHA passes Rust build/lint/test gates, adapter/domain/API/HTML tests and unchanged Elixir regression gates and browser E2E; zero critical/high/medium defects | QA 1/2 |

Use isolated disposable PostgreSQL schemas/databases for integration tests; real asynchronous tasks and subprocesses for lifecycle tests; deterministic app-server fixtures for offline E2E through actual dynamic-tool execution. Add browser automation as a locked test-only dependency with an explicit CI command before stage 3 completion. Rust gates: cargo fmt --all -- --check; cargo clippy --workspace --all-targets --all-features -- -D warnings; cargo test --workspace --all-features; cargo build --release --locked. Record explicit SQL migration, browser E2E and dependency audit commands once tools are selected. Require no unresolved critical/high/medium security findings and reviewed license compatibility. Preserve the reference Elixir gate (make all) and its configured coverage threshold; report any missing reference toolchain as a blocker, not a pass. Rust modules covering authorization, transitions, identifiers, scheduler state and adapter normalization require explicit positive/negative and race/failure tests; PM/QA establish a coverage baseline without claiming Elixir's threshold measures Rust.

Stage 0 precedes all feature integration: inventory SPEC sections 4–18 and Appendix A against the source, classify required/recommended/existing extensions, and create a versioned parity matrix with fixtures, target modules and tests. Cover CLI flags/config defaults/aliases, prompt rendering, path/hooks, scheduler dispatch/retry/reconciliation, local and SSH app-server wire messages, token accounting, blocked approvals, HTTP response shapes, and all five adapters. Golden fixtures must be sanitized. Run the same scenarios against reference and Rust and explain every difference; no undocumented semantic drift is accepted. Standard SPEC non-goals do not preclude this fork's product UI extension.

Existing live provider E2E creates remote resources and may incur model costs: do not run without budget/access authorization. Mandatory offline contract/regression tests must not depend on those services. Clearly list live suites unrun. Build and smoke Rust artifacts on the existing macOS/Linux arm64/x86_64 release targets before advertising equivalent packaging; preserve the existing Burrito workflows until Rust acceptance. Runtime fallback is a deliberate operator choice, never two orchestrators polling the same scope concurrently.
Release discipline: isolated ticket branches/worktrees from Operations integration branch; no edits in another agent's working tree. PM routes each iteration to Operations for reviewed checkpoint diff/commit and push when remote is available. QA tests the integrated SHA, never a mix of branch claims. Two failed fix/retest cycles for the same defect escalate to CTO without gate relaxation. Accepted sprint requires all criteria/checks and zero critical/high/medium defects; document low defects with owner/disposition. Operations then records changelog/version metadata, release commit/tag and push. Version belongs in metadata/tag; commit subject need not include it. CEO owns final acceptance/report. GitHub fork/auth blockage prevents push/publication, not local checkpoint commits; report local-only evidence explicitly until resolved by the authorized account owner.

## 10. Evidence and remaining uncertainty

Design inspection verified source HEAD, untracked CEO HLD, elixir/AGENTS.md, READMEs, logging rules, Tracker callbacks/session binding, Issue fields, Linear adapter, DynamicTool, AppServer issue option flow, router, build/dependency definitions, SPEC overview/conformance section index and existing release workflow filenames. The reference has no SQL persistence or user authentication boundary. SPEC explicitly supports language-independent implementation. The user then requested Rust; this revision changes implementation language and architecture accordingly without reducing functional scope.

This is design-only delivery. No application implementation, tests, dependency installation, benchmark, security verdict, GitHub authentication check or deployment was performed. GitHub auth failure is requester-reported. Operations/PM must preserve the HLD's currently untracked content before worktree creation; HEAD alone does not contain it. New dependencies, prompt-template parity, all provider ports, worker isolation, runtime lifecycle recovery and four-platform packaging are explicit implementation risks. PM must re-estimate the runtime port rather than reuse an incremental Phoenix schedule. No genuine staffing gap is established, but stage 0 must assess Rust competence and delivery estimates before committing dates.

CEO retains HLD scope authority; current user direction supersedes its Elixir/Phoenix boundary. Adjacent RUST_CHANGE_REQUEST.md records the requested HLD amendment; original HLD is retained unchanged for provenance. CTO owns technical changes; PM routes implementation/QA/Operations and returns concrete ambiguities or repeated failures. No additional approval is needed to adopt the user's Rust request.


## 11. Consolidated R03 decisions (normative)

The following decisions supersede conflicting detail above.

# R03 CTO decisions — Rust LLD revision 2 addendum

Date: 2026-09-25. Project: Symphony Rust fork. S0/R03.
Applies to PROJECT_LLD.md revision 2; this addendum controls ambiguities below.
Reviewed docs/R03_ADVERSARIAL_REVIEW.md in Operations canonical checkout at
2d5d6dce6627d3d6f373f5cd991cb2fd84c5de68; checkout was clean. Review is design evidence,
not an implementation or independent QA verdict. PM imports decisions into tickets;
Operations checkpoints this document; QA1/2 own independent tests T01–T13.

## 1. Atomic configuration and scope

The single scheduler owner serializes accepting a validated immutable config generation
and admitting claims. Each claim captures generation ID, adapter, org/project/assignee
scope and host-side credential references; runner/tool binding inherits it. Each async
read carries its generation; results from a superseded generation cannot dispatch work.
Reject kind/org/project/assignee changes while any running, retrying or blocked claim
exists. Reject the entire reload, retaining all prior settings. Existing session tool
bindings retain prior credentials on credential-only reload. Credentials remain host-side.

Drain means stop new admissions, cancel retries, explicitly cancel blocked/running
attempts and await process termination before releasing claims. A failed cancellation
keeps the scope switch blocked. Operator drain/resume is an authenticated admin action
(or local operator control); it does not alter ticket states. Startup/restart may not
assume orphan workers disappeared. No silent forced scope change.

## 2. Built-in agent transition authority

Add explicit tracker.provider.agent_transition_states (state UUID allowlist); default
empty denies transitions. Setup documentation demonstrates enabling In Progress and a
human-review state. An admin may explicitly enable terminal targets such as Done;
terminal workflow state never constitutes this delivery team's independent QA acceptance.
No automatic permission to cancel, archive, reassign, change blockers or edit other tickets.
Humans retain authorized direct state changes; a configurable transition graph is outside
MVP. Host attempt identity and scope are authoritative, never tool JSON.

Every mutation, including a replay, checks active attempt authority and ticket membership
inside the transaction. Reject moved/out-of-scope tickets with 404; archived, terminal,
newly blocked, disabled-assignee or assignment-changed tickets with a safe conflict.
Capture initial assignee even if scheduler assignee filter is absent. Do not let a stale
attempt mutate after reassignment. Terminal targets are allowed only from nonterminal
state and explicit allowlist; result replay after terminal completion is denied by the
fresh authority check. This deliberate fail-closed outcome is documented to tool callers.

## 3. Authorization and revocation

Session lookup, user enabled/role checks, scope checks and mutation occur in one DB
transaction. Mutations lock user then session rows; account disable/session revocation
uses the same order. A mutation that obtains those locks before revocation may commit
first; revocation cannot undo it. After revocation commits, newly admitted mutations fail.
All related rows require matching organization, and required project/team relationships.
Out-of-scope lookup returns 404. Never serialize hidden row details into errors.

SSE carries invalidation identifiers only, not sensitive record bodies. Check current
session/user authority on connect and before each send; revocation broadcasts close
connections, backed by a maximum 15-second revalidation heartbeat. Already-sent network
bytes cannot be recalled. Do not promise instantaneous revocation of in-flight bytes;
subsequent data fetches must reauthorize. QA T04 tests this explicit boundary.

## 4. Domain writes, cancellation and stale events

Built-in mutation transaction locks project/ticket and checks blockers plus current
attempt authority. Host maintains a DB attempt fence for built-in writes; invalidating
domain operations fence that attempt in the same transaction, then request cancellation.
For external trackers, scheduler attempt generation remains the authority for local
runtime events; no new provider write semantics are invented.

Archive, new blockers, reassignment and terminal transition invalidate current built-in
mutation authority. A transition that commits before invalidation is valid; no later
mutation may commit under the old fence. Runtime completion can record the old attempt's
historical outcome but cannot revive claims, imply ticket acceptance or clean a replacement
workspace. Serialize workspace use/cleanup by issue with attempt ownership checks; do not
start a replacement until old process termination and cleanup safety are established.
Transport/database read errors preserve claims pending retry/reconciliation, never imply
empty tracker results. Generation checks protect late retry timers and callbacks too.

## 5. Idempotency and lock order

Persist original HTTP status, sanitized response, resulting resource version and canonical
request hash atomically with activity and mutation. A valid same-body replay returns that
stored result before applying stale lock_version checks, but only after fresh authorization
and attempt-fence checks. Changed body returns 409. Scope keys by stable user UUID or agent
attempt UUID plus operation and ticket; user session renewal does not change user scope.
A different attempt cannot replay the prior attempt's authority. Retain keys seven days;
a retry beyond retention is a new operation and must pass concurrency checks.

Order locks consistently: user, session, project UUIDs sorted, ticket UUIDs sorted, then
relationship/attempt/idempotency rows. Agent calls omit human session locks. Dependency
changes and relevant archives serialize under the project lock. Concurrent duplicate keys
use uniqueness plus bounded transaction retry; never return an uncommitted result. Retry
deadlocks/serialization errors at most three times with jitter, then return retryable 503.
No network I/O while these DB locks are held.

## 6. Production filesystem and secret isolation

Service identity owns configuration/secret directories mode 0700 and secret files 0600;
worker identity has no group/ACL access. Backups and service logs follow that boundary.
Workers get only dedicated workspace mounts and required runtime tools, no service home,
source checkout, Docker socket, database socket, backup mount or host process namespace.
Use isolated worker containers or a separate host account with enforced filesystem/process
restrictions; production readiness requires a tested deployment profile, not file modes
alone. Database network/auth policy denies workers. Close inherited FDs except documented
stdio/protocol channels; never carry secrets in argv or prompts. Hooks/bootstrap execute
with the worker restrictions, not service privileges.

Required agent-provider authentication is a separate narrowly scoped worker credential,
mounted read-only in a worker-only secret path or provided by an approved credential proxy.
Do not copy the service's entire home/auth directory. Its allowed use is distinct from
service database/session/tracker/SSH secrets. Service-side SSH private key remains 0600,
no agent forwarding; remote worker cannot read it. Local development may explicitly use
a weaker trusted profile but cannot satisfy production isolation tests T10/T11.

## 7. SSH and process lifecycle

Use a fixed versioned remote helper protocol over SSH stdin; encode structured arguments
as JSON, validate them and spawn without evaluating remote shell interpolations. The
fixed helper command/path is administrator-configured, never ticket-controlled. Enforce
host-key checking against a preprovisioned known_hosts file; unknown/mismatched keys fail.
Disable agent forwarding, arbitrary environment forwarding and interactive prompts.
Allow only documented host configuration such as destination/port/identity/known_hosts;
reject custom ProxyCommand/LocalCommand injection through product config. Environment
allowlist is explicit and contains no service secrets.

Helper starts an attempt-owned process group (Unix target platforms), stores protected
attempt ownership metadata and supports authenticated cancel/status. Send TERM, wait up
to 10 seconds, then KILL and wait up to 5 seconds. Failure to confirm exit marks the
attempt uncertain/interrupted, blocks workspace reuse and exposes an operator action;
it is never success. SSH disconnect triggers cancellation/reconciliation, not completion.
Worker lease heartbeat every 5 seconds expires after 30 seconds; helper terminates its
process group on lease loss. On restart, reconcile persisted helper attempt identity and
lease status before cleanup/retry. Never kill by unvalidated reused PID or delete a path
solely from stale metadata. Test parent/helper/transport failure independently. This is a
Rust hardening extension and must be labeled separately from reference parity fixtures.

## 8. HTTP edge policy

Production cookies Secure + HttpOnly + SameSite=Lax; loopback-only development exception
requires explicit configuration. Trust forwarded scheme/address only from configured proxy
CIDRs; otherwise ignore forwarded headers. Validate configured public origin/hosts and use
relative same-origin redirect targets. Rotate session ID and CSRF token at login/privilege
change; logout revokes session. Check CSRF on every cookie-authenticated write including
observability refresh. Auth middleware covers every PM-enabled observability route.

Login throttle per normalized account and effective client address, with bounded storage
and generic errors: initial defaults 5 failed attempts per account per 15 minutes and 30
per address per 15 minutes; admin-configurable. Never use permanent account lockout.
SSE origin must match configured origin when supplied, with authenticated same-origin
requests only and no permissive CORS; apply decision 3 on every stream. Validate proof
links as data under LLD URL rules, never derive links from untrusted Host/forwarded headers.

## Stage 0 acceptance boundary and status

Confirm PM interpretation: R01 requires reviewed traceability covering required SPEC and
existing supported extensions, correctly attributed reference/LLD-only fixtures, portable
structural validation and planned executable tests. Full Rust/reference equivalence is
required later, before acceptance/release, not before creating the Rust runtime. A raw
count of source assertions is not a completeness proof or an additional acceptance rule.

R02 still requires a real bounded Rust build, resolved lockfile and dependency/license
review. Missing crates/DNS is an environment blocker, not proof of skill failure and not
permission to waive the gate or buy infrastructure. PM can route independent fixture,
design and documentation work while build feasibility is blocked. No new staffing gap
is established. R03 decisions are technically resolved by this document, but QA review
and Operations preservation remain required; implementation tests are not yet run.

Four QA2 Medium fixture defects remain open until independent QA2 retest of the repaired
integrated SHA. Repair cycle 1 has been supplied; do not count structural validation as
an independent retest. Operations imports reviewed repair artifacts and this addendum,
then QA2 retests, then PM updates disposition. All original release gates remain intact.


## 12. Migration, defect carryover and final delivery contract

Product name is Symphosium. The existing HLD's Elixir/Phoenix implementation boundary is superseded by the user's Rust request and this LLD; functional scope is preserved. CEO owns updating HLD wording. Do not overwrite the original HLD while checkpointing this design.

Migration sequence: inventory the current WORKFLOW configuration and validate it without running agents; create/backup the built-in database; run versioned migrations; idempotently seed the configured organization/team/states and provision admin; test the Rust configuration using a disposable project; drain and stop the Elixir instance; verify worker termination; start exactly one Rust scheduler for the production scope. External tracker records remain in their providers and IDs/config aliases stay stable. Do not silently import or copy external tickets into the built-in database. A bulk-import utility is outside MVP. There is no reference application database to convert. Existing workspace reuse requires verified ownership/path compatibility and no live previous attempt. Rollback stops/drains Rust before restoring the compatible prior binary/config; preserve database backups and do not drop user data. A deployment already using built-in tickets cannot schedule those tickets through the old Elixir runtime; use the previous accepted Rust binary for that scope.

Known carryover defects, not yet independently closed:

| Defect | Severity | Subject | Required disposition |
| --- | --- | --- | --- |
| QA2-R01-FX-001 | Medium | Config alias/map fixture fidelity | Integrate Senior1 repair; QA2 verifies actual reference anchors |
| QA2-R01-FX-002 | Medium | Fragmented app-server line attribution | QA2 distinguishes reference complete-line assertion from LLD-only pre-newline test |
| QA2-R01-FX-003 | Medium | Asana/GitLab normalized-field fidelity | Integrate Mid1 repairs; QA2 verifies fields |
| QA2-R01-FX-004 | Medium | Jira/Asana paging/refresh transcripts | QA2 verifies page and refresh scenarios |

The prior discovery work at 2d5d6dce6627d3d6f373f5cd991cb2fd84c5de68 in a separate Operations checkout is not integrated into this canonical repository. Its fixture repair paths and exact sequence are recorded in /Users/benblum/.openmausbot/task-workspaces/6df7490b-cec7-423c-bf28-7d7b940d8583/4070c066-23c3-4169-b938-37e0126c9502/symphony-linear-delivery/DEFECT_ROUTING.md. Reuse reviewed artifacts rather than recreating them. Four defects remain open until QA2 retests the resulting integrated SHA. Earlier structural reports of 38 runtime/40 provider records do not prove fidelity or runtime parity. R03 decisions are included in section 11 and no longer pending design resolution. Prior crates.io DNS/cache and room handoff budget failures are historical blockers to recheck when attempting the relevant action, not facts about current availability.

Permanent staffing remains sufficient as a planning assumption; competence/throughput must be established through the bounded Rust build and reviews. No hire or paid service is authorized. PM owns tickets/Gantt, assignments, defect state and implementation routing; Operations owns every checkpoint and final commit; QA owns independent integrated verdicts. Preserve the existing source, user files and unrelated work.

Final completion requires actual Rust build/lint/unit/integration/browser acceptance outputs tied to the same integrated revision, all listed acceptance criteria, independent QA with zero critical/high/medium defects, documented low defects, an Operations commit in the canonical repository, and a clean integrated worktree verified with git status. Record the complete commit hash, exact commands/results and evidence artifacts. A commit is distinct from a push. Operations may push only to verified origin symphosium with working credentials; never push fork work to upstream. Do not label release published without remote verification. A local design document, fixture corpus, dependency spike or planning handoff alone does not satisfy the requested committed rewrite.
