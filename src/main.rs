use actix_web::{web, App, HttpServer};

use util::constants::EnvironmentVariable;

use crate::repository::Repo;

mod repository;
mod routes;

pub mod models;
mod util;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let repo = Repo::init().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .service(web::resource("/").to(|| async { "{ message: \"hoaq vu to\" }" }))
            .configure(routes::init)
    })
    .bind(EnvironmentVariable::ServerHost.value())?
    .run()
    .await
}
