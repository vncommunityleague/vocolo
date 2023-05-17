use std::net::SocketAddr;
use axum::Router;
use reqwest::get;
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
        .route("/", get(|| async { "{ message: \"hoaq vu to\" }" }))
        .with_state(repo)
        ;

    routes::init(&app).await;
    axum::Server::bind(&SocketAddr::from(EnvironmentVariable::ServerHost.unwrap()))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
