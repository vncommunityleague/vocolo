use actix_web::{App, HttpServer, web};
use crate::constants::EnvironmentVariable;

mod helper;
mod models;
mod repository;
mod routes;

mod constants;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    helper::init().await;

    HttpServer::new(move || {
        App::new()
            .configure(routes::init)
    }).bind((EnvironmentVariable::SERVER_HOST.value(), EnvironmentVariable::SERVER_PORT.value_with_type::<u16>()))?
        .run()
        .await
}
