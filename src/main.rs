use sea_orm::{ConnectOptions, Database};
use tracing::info;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use vocolo_migration::{Migrator, MigratorTrait};

use crate::routes::AppState;

pub mod error;
pub mod models;

mod routes;
mod util;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "api_logging=info,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = util::env::var("DATABASE_URL");
    let db_min_connections = util::env::var("DATABASE_MIN_CONNECTIONS")
        .parse::<u32>()
        .expect("DATABASE_MIN_CONNECTIONS must be a number");
    let db_max_connections = util::env::var("DATABASE_MAX_CONNECTIONS")
        .parse::<u32>()
        .expect("DATABASE_MAX_CONNECTIONS must be a number");

    info!("Connecting to database at {}", &db_url);
    let mut db_opt = ConnectOptions::new(db_url);
    db_opt
        .min_connections(db_min_connections)
        .max_connections(db_max_connections);

    let conn = Database::connect(db_opt)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&conn, None).await.unwrap();

    let host = util::env::var("HOST_ADDRESS");
    let state = AppState { database: conn };

    info!("Starting server at {}", &host);
    let app = routes::init(state);
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    axum::serve(listener, app).await.expect("server failed");
}
