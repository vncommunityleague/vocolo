use crate::constants::EnvironmentVariable;
use actix_web::{web, App, HttpServer};

mod helper;
mod models;
mod repository;
mod routes;

mod constants;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(helper::osu::OsuHelper::init()))
            .configure(routes::init)
    })
    .bind((
        EnvironmentVariable::SERVER_HOST.value(),
        EnvironmentVariable::SERVER_PORT.value_with_type::<u16>(),
    ))?
    .run()
    .await
}
