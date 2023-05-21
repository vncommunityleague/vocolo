use std::net::{ToSocketAddrs};

use axum::{Router, ServiceExt};
use axum::routing::get;

use util::constants::EnvironmentVariable;

use crate::repository::Repo;

mod repository;
mod routes;

pub mod models;
mod util;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let repo = Repo::init().await;

    let app = Router::new()
        .route("/", get(|| async { "hoaq vu to!" }))
        .merge(routes::init_routes())
        .with_state(repo);

    axum::Server::bind(&EnvironmentVariable::ServerHost
            .value()
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
