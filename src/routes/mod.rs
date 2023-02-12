use actix_web::web;

mod osu;

mod users;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("osu").configure(osu::init));
}
