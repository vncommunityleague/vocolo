use actix_web::web;

mod auth;
mod osu;
mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("authorize").configure(auth::config));
    cfg.service(web::scope("osu").configure(osu::config));
}
