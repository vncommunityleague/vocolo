use std::net::{SocketAddr, ToSocketAddrs};

use axum::Router;

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
        // .route("/", get(|| async { Ok(Response::new(StatusCode::OK))) })
        .with_state(repo)
        .nest("/", routes::init_routes());

    axum::Server::bind(&SocketAddr::from(
        EnvironmentVariable::ServerHost
            .value()
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap(),
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();
}
