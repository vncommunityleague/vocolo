use sea_orm::{ConnectOptions, Database};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use vocolo_core::env_var;
use vocolo_models::{Migrator, MigratorTrait};

use crate::route::AppState;

mod error;
mod route;

pub type Result<T> = std::result::Result<T, error::Error>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "api_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = env_var("DATABASE_URL");
    let host = env_var("HOST_ADDRESS");

    let db_min_connections = env_var("DATABASE_MIN_CONNECTIONS").parse::<u32>().unwrap();
    let db_max_connections = env_var("DATABASE_MAX_CONNECTIONS").parse::<u32>().unwrap();

    let mut db_opt = ConnectOptions::new(db_url);
    db_opt
        .min_connections(db_min_connections)
        .max_connections(db_max_connections);

    let conn = Database::connect(db_opt).await.unwrap();

    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { database: conn };

    let app = route::init(state);
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    axum::serve(listener, app).await.expect("server failed");
}
