use actix_web::web;

mod auth;
mod osu;
mod users;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("auth").configure(auth::config));
    cfg.service(web::scope("osu").configure(osu::config));
}
