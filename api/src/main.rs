use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::AppState;

mod routes;

#[tokio::main]
async fn main() {
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

    let db = vocolo_database::connect_to_db()
        .await
        .expect("Failed to connect to database");

    let host = std::env::var("HOST_ADDRESS").unwrap_or("0.0.0.0:8080".to_owned());
    let state = AppState { db };

    info!("Starting server at {}", &host);
    let app = routes::init(state);
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    axum::serve(listener, app).await.expect("server failed");
}
