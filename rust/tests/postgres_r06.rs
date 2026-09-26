//! Explicit disposable-PostgreSQL acceptance checks for R06.
//!
//! Invoke with `PM_TEST_DATABASE_URL=<fresh disposable URL> cargo test --locked
//! r06_postgres -- --ignored`. The test intentionally errors when the variable is
//! absent or the database cannot be reached; it is never a pass-through skip.
use std::env;
use symphosium::db::{Database, MIGRATOR};
use uuid::Uuid;

fn database_url() -> String {
    env::var("PM_TEST_DATABASE_URL").expect(
        "PM_TEST_DATABASE_URL is required and must name a fresh disposable PostgreSQL database",
    )
}

async fn reset(database: &Database) {
    // A fresh disposable database is mandatory. Undo then up proves the migration's
    // reversible path; a failed undo/up is a test failure, never a skipped result.
    database.migrate().await.unwrap();
    MIGRATOR.undo(database.pool(), 0).await.unwrap();
    database.migrate().await.unwrap();
}

async fn project_fixture(database: &Database, slug: &str) -> (Uuid, Uuid, Uuid, Uuid) {
    let org = Uuid::new_v4();
    let team = Uuid::new_v4();
    let project = Uuid::new_v4();
    let state = Uuid::new_v4();
    sqlx::query("INSERT INTO organizations(id, slug, name) VALUES ($1, $2, 'Organization')")
        .bind(org)
        .bind(format!("{slug}-{org}"))
        .execute(database.pool())
        .await
        .unwrap();
    sqlx::query("INSERT INTO teams(id, organization_id, name, key) VALUES ($1, $2, 'Team', 'TST')")
        .bind(team)
        .bind(org)
        .execute(database.pool())
        .await
        .unwrap();
    sqlx::query("INSERT INTO projects(id, organization_id, team_id, name, slug) VALUES ($1, $2, $3, 'Project', $4)")
        .bind(project).bind(org).bind(team).bind(format!("project-{project}")).execute(database.pool()).await.unwrap();
    sqlx::query("INSERT INTO workflow_states(id, organization_id, team_id, name, category, position) VALUES ($1, $2, $3, 'Todo', 'backlog', 10)")
        .bind(state).bind(org).bind(team).execute(database.pool()).await.unwrap();
    (org, team, project, state)
}

#[tokio::test]
#[ignore = "requires PM_TEST_DATABASE_URL for a fresh disposable PostgreSQL database"]
async fn r06_postgres_migration_seed_and_constraints() {
    let database = Database::connect(&database_url())
        .await
        .expect("connect disposable PostgreSQL");
    reset(&database).await;
    let (org, team, project, state) = project_fixture(&database, "r06").await;

    database.seed_standard_states(org, team).await.unwrap();
    database.seed_standard_states(org, team).await.unwrap();
    let seeded: (i64,) = sqlx::query_as("SELECT count(*) FROM workflow_states WHERE team_id=$1")
        .bind(team)
        .fetch_one(database.pool())
        .await
        .unwrap();
    assert_eq!(
        seeded.0, 5,
        "fixture Todo plus four remaining idempotent standard states"
    );

    let first = database
        .create_ticket(org, team, project, state, "first")
        .await
        .unwrap();
    let second = database
        .create_ticket(org, team, project, state, "second")
        .await
        .unwrap();
    let identifiers: Vec<(String,)> =
        sqlx::query_as("SELECT identifier FROM tickets WHERE id = ANY($1) ORDER BY number")
            .bind(vec![first, second])
            .fetch_all(database.pool())
            .await
            .unwrap();
    assert_eq!(identifiers, vec![("TST-1".into(),), ("TST-2".into(),)]);

    let (other_org, other_team, other_project, other_state) =
        project_fixture(&database, "other").await;
    let cross_org = database
        .create_ticket(org, other_team, other_project, other_state, "wrong scope")
        .await;
    assert!(
        cross_org.is_err(),
        "cross-organization create must be rejected"
    );
    let other_ticket = database
        .create_ticket(other_org, other_team, other_project, other_state, "other")
        .await
        .unwrap();
    assert!(
        database
            .add_dependency(org, first, other_ticket)
            .await
            .is_err(),
        "cross-organization dependency must be rejected"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires PM_TEST_DATABASE_URL for a fresh disposable PostgreSQL database"]
async fn r06_postgres_concurrent_identifier_and_inverse_cycle() {
    let database = Database::connect(&database_url())
        .await
        .expect("connect disposable PostgreSQL");
    reset(&database).await;
    let (org, team, project, state) = project_fixture(&database, "race").await;

    let mut creates = Vec::new();
    for index in 0..16 {
        let database = database.clone();
        creates.push(tokio::spawn(async move {
            database
                .create_ticket(org, team, project, state, &format!("ticket {index}"))
                .await
                .unwrap()
        }));
    }
    let mut created = Vec::new();
    for task in creates {
        created.push(task.await.unwrap());
    }
    let count: (i64,) =
        sqlx::query_as("SELECT count(DISTINCT identifier) FROM tickets WHERE project_id=$1")
            .bind(project)
            .fetch_one(database.pool())
            .await
            .unwrap();
    assert_eq!(count.0, 16, "concurrent allocation must not collide");

    let left = created[0];
    let right = created[1];
    let forward = {
        let database = database.clone();
        tokio::spawn(async move { database.add_dependency(org, left, right).await })
    };
    let reverse = {
        let database = database.clone();
        tokio::spawn(async move { database.add_dependency(org, right, left).await })
    };
    let results = [forward.await.unwrap(), reverse.await.unwrap()];
    assert_eq!(
        results.iter().filter(|result| result.is_ok()).count(),
        1,
        "one inverse edge must lose"
    );
    assert_eq!(
        results.iter().filter(|result| result.is_err()).count(),
        1,
        "cycle must be rejected"
    );
}
