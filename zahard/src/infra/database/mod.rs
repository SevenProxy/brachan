mod migrations;

use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;

pub use migrations::Migrator;

/// Connects to the database and runs pending migrations.
///
/// Already-applied migrations (tracked in `seaql_migrations`) are skipped.
pub async fn connect(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(url).await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}
