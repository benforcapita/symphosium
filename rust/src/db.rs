//! PostgreSQL primitives for the built-in project-management mode.
//!
//! Migrations are an explicit release operation: [`Database::connect`] never runs them.
//! Ticket creation and dependency changes use database transactions so the constraints in
//! `migrations/` remain authoritative under concurrent requests.
use sqlx::{PgPool, Postgres, Transaction, postgres::PgPoolOptions};
use uuid::Uuid;

pub const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("database error")]
    Sql(#[from] sqlx::Error),
    #[error("migration error")]
    Migration(#[from] sqlx::migrate::MigrateError),
}

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Opens a pool only. Call `migrate` from an explicit release/admin action.
    pub async fn connect(url: &str) -> Result<Self, DatabaseError> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(10)
                .connect(url)
                .await?,
        })
    }
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
    pub async fn migrate(&self) -> Result<(), DatabaseError> {
        Ok(MIGRATOR.run(&self.pool).await?)
    }

    /// Idempotently creates the five standard workflow states for one team.
    pub async fn seed_standard_states(
        &self,
        organization_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), DatabaseError> {
        sqlx::query("SELECT seed_standard_workflow_states($1, $2)")
            .bind(organization_id)
            .bind(team_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Allocates an immutable team identifier while holding the team row lock.
    pub async fn create_ticket(
        &self,
        organization_id: Uuid,
        team_id: Uuid,
        project_id: Uuid,
        state_id: Uuid,
        title: &str,
    ) -> Result<Uuid, DatabaseError> {
        let mut tx = self.pool.begin().await?;
        let id = create_ticket_tx(
            &mut tx,
            organization_id,
            team_id,
            project_id,
            state_id,
            title,
        )
        .await?;
        tx.commit().await?;
        Ok(id)
    }

    /// Adds a same-project acyclic dependency. The migration function locks project then tickets.
    pub async fn add_dependency(
        &self,
        organization_id: Uuid,
        ticket_id: Uuid,
        blocker_id: Uuid,
    ) -> Result<(), DatabaseError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT add_ticket_dependency($1, $2, $3)")
            .bind(organization_id)
            .bind(ticket_id)
            .bind(blocker_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}

async fn create_ticket_tx(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    team_id: Uuid,
    project_id: Uuid,
    state_id: Uuid,
    title: &str,
) -> Result<Uuid, sqlx::Error> {
    let row: (Uuid,) = sqlx::query_as("SELECT create_ticket($1, $2, $3, $4, $5)")
        .bind(organization_id)
        .bind(team_id)
        .bind(project_id)
        .bind(state_id)
        .bind(title)
        .fetch_one(&mut **tx)
        .await?;
    Ok(row.0)
}
