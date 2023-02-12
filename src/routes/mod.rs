use actix_web::web;

mod osu;

mod auth;
mod users;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("authorize")
            .service(auth::osu_login)
            .service(auth::osu_login_callback),
    );

    cfg.service(web::scope("osu").configure(osu::init));
}
