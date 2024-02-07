use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(vocolo_database_migrations::Migrator).await;
}
