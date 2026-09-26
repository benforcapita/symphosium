# R06 database foundation evidence

Base: `23b3f7f1a14eca8ed27b3812d9db10437149fb98`. This change adds the explicit SQLx/PostgreSQL migration boundary only; it does not enable builtin mode, run migrations at boot, or implement R08 services/API/authentication.

`rust/migrations/0001_project_management.up.sql` creates organization-scoped core entities, composite ownership constraints, identifier allocation, idempotent workflow-state seeding, and same-project acyclic dependencies. `0001_project_management.down.sql` reverses those objects in dependency order. `rust/src/db.rs` exposes explicit `Database::migrate`, `seed_standard_states`, ticket allocation, and dependency insertion; `Database::connect` deliberately does not migrate.

Verified local checks on 2026-09-26, using Cargo 1.97.1 and
`CARGO_TARGET_DIR=/private/tmp/symphosium-r06-target`:

- `cargo fmt --manifest-path rust/Cargo.toml --all -- --check` passed.
- `cargo clippy --manifest-path rust/Cargo.toml --locked --offline --all-targets -- -D warnings` passed.
- `cargo test --manifest-path rust/Cargo.toml --locked --offline --all-targets` passed: 21 non-ignored tests.
- `cargo test --manifest-path rust/Cargo.toml --locked --offline --release --all-targets` passed: 21 non-ignored tests.
- With the supplied disposable PostgreSQL 16 database, `PM_TEST_DATABASE_URL=… cargo test --manifest-path rust/Cargo.toml --locked --offline r06_postgres -- --ignored` passed: 2 integration tests. These prove migration up/down/up, idempotent seed, concurrent identifier allocation, scope rejection, and competing inverse dependency rejection.

The integration test requires host-network permission when executed in a sandbox because it opens the local PostgreSQL port. It fails explicitly rather than treating an unavailable database as a passing skip.
