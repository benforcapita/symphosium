# Symphosium R06-REV1 rolling technical review

Date: 2026-09-26. Source: `ops/r06-database-wip` at `6ef44a14314e819174d6a0175d1ea0a0c94e1d86`, based on R05 accepted `23b3f7f1a14eca8ed27b3812d9db10437149fb98`. Review clone: `r06-contract-review`, branch `review/r06-r09-contracts`. This reviews the bounded R06 WIP database snapshot; it is not QA acceptance. No source or fixture JSON was changed.

## Findings

1. **Medium — same-project dependency is not a database invariant.** `rust/migrations/0001_project_management.up.sql:13` gives each endpoint an organization-scoped FK, but does not constrain their `project_id` values. The `add_ticket_dependency` function at lines 38–51 checks this correctly; direct insertion through `Database::pool()` or later R08 service SQL can bypass it. `R06_REV1_CONTRACT_REPRO.sql` inserted an edge joining two tickets in different projects of the same organization; `cross_project_dependency_accepted = 1`. LLD3 §3 requires same-project MVP dependencies and says organization/project relationships are enforced by constraints or transactional context checks. Require every dependency write path to use the guarded transaction function or an equivalent guarded service boundary, prevent direct writes outside it, and test both legitimate same-project insertion and cross-project rejection. CTO should decide whether database privileges/trigger or an exclusive service boundary owns this invariant before R08.

2. **Medium — identity columns declared immutable can be rewritten.** The schema permits `UPDATE teams SET key='NEW'` and `UPDATE tickets SET identifier='REWRITTEN-999'`; both succeeded in the fixture. The create function locks and allocates correctly, but no update guard exists in R06. LLD3 §3 requires immutable team key and ticket identifier, and stable identifiers never reused. A later administrative update or R08 generic patch could silently break links and permit identifier reuse. Add a positive same-team project move test plus negative key/identifier/cross-team mutation tests at the chosen service or DB boundary; make the SQL permission/trigger contract explicit.

3. **Medium — seed silently retains incompatible existing standard state.** `seed_standard_workflow_states` uses `ON CONFLICT (team_id,name) DO NOTHING` (migration lines 53–57). A pre-existing `Todo` row with `category='terminal', position=99` remains unchanged after seed; the fixture reports exactly that. LLD3 §3 defines `Todo` as a standard backlog state and requires category validation against configured active/terminal names. Seed should validate existing standard names/categories and fail with a safe, actionable error or reconcile them under an explicit CTO policy. Test an existing conflicting name plus successful idempotent rerun. This can affect R07/R08 setup and later dispatch decisions.

## R07 readiness and coverage

The `users` and `sessions` composite FK ties session user to organization; token hashes are stored as `bytea` with uniqueness. That schema is a usable starting point for R07 auth. R07 still needs its own transaction contract: user then session row locks, fresh enabled/role/scope checks, revocation race, CSRF, and safe errors per LLD3 §11.3. The R06 `Database::pool()` exposes unrestricted SQL, so R07/R08 must state which layer exclusively owns writes to identity and relationships. This review does not block isolated R07 implementation against the current schema; it does block treating R06 invariants as complete or integrating R07 on the assumption they are enforced. CTO owns the boundary decision, Mid1 owns R06 repair, PM routes, QA2 independently verifies an integrated SHA.

Existing R06 integration tests cover migration up/down/up, idempotent seed with a matching Todo row, concurrent 16-ticket creation, scoped create rejection, cross-organization dependency rejection, and one inverse-edge race. They do not cover the three cases above, same-team project moves, or a multi-hop cycle. The attached SQL is a focused, rollback-only reproduction; turn the cases into disposable PostgreSQL regression tests once the contract is fixed. No new architecture is prescribed here.

## Independent commands and results

Toolchain: `cargo 1.97.1 (c980f4866 2026-06-30)` from `/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin`; local Docker 29.5.2; cached `postgres:16` image `1a6ab3f5345e`. Commands run in the isolated review clone unless stated otherwise:

```sh
PATH=/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH CARGO_TARGET_DIR=/private/tmp/symphosium-r06-review-target cargo fmt --manifest-path rust/Cargo.toml --all -- --check
PATH=/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH CARGO_TARGET_DIR=/private/tmp/symphosium-r06-review-target cargo test --manifest-path rust/Cargo.toml --locked --offline --all-targets
docker run --rm -d --name symphosium-r06-review-20260926 -e POSTGRES_HOST_AUTH_METHOD=trust -e POSTGRES_DB=r06_review -p 127.0.0.1:55439:5432 postgres:16
docker exec -i symphosium-r06-review-20260926 psql -U postgres -d r06_review -v ON_ERROR_STOP=1 < rust/migrations/0001_project_management.up.sql
docker exec -i symphosium-r06-review-20260926 psql -U postgres -d r06_review -v ON_ERROR_STOP=1 -q < ../R06_REV1_CONTRACT_REPRO.sql
docker exec symphosium-r06-review-20260926 psql -U postgres -d postgres -v ON_ERROR_STOP=1 -c 'CREATE DATABASE r06_test'
PATH=/Users/benblum/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH CARGO_TARGET_DIR=/private/tmp/symphosium-r06-review-target PM_TEST_DATABASE_URL=postgres://postgres@127.0.0.1:55439/r06_test cargo test --manifest-path rust/Cargo.toml --locked --offline --test postgres_r06 r06_postgres -- --ignored --test-threads=1
```

Results: fmt passed; 21 nonignored Rust tests passed, two PostgreSQL tests ignored in that command. The first PostgreSQL test attempt against `r06_review` failed because I had already applied the migration manually, violating its fresh-DB precondition. The corrected run against fresh `r06_test` passed 2/2. The reproduction SQL completed successfully and printed `Todo | terminal | 99`, `cross_project_dependency_accepted = 1`, `mutable_team_key = NEW`, and `mutable_ticket_identifier = REWRITTEN-999`. The transaction rolled back. No live provider/model calls or remote push occurred.

R09 repair was not supplied with this review snapshot, so R09 re-review remains pending a precise repaired SHA. R06 and R09 review capacity is reserved on this branch. Next owner: PM for routing the three bounded fixes/decision to Mid1/CTO, then Operations checkpoint and QA2 independent test on the integrated revision.
