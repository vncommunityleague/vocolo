use actix_web::{App, HttpServer, web};

use common::{constants::EnvironmentVariable, helper::osu::OsuHelper};

mod repository;
mod routes;

mod util;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(OsuHelper::init()))
            .configure(routes::init)
    })
        .bind((
            EnvironmentVariable::SERVER_HOST.value(),
            EnvironmentVariable::SERVER_PORT.value_with_type::<u16>(),
        ))?
        .run()
        .await
}
