use actix_web::web::ServiceConfig;

mod mappools;
mod matches;
mod players;
mod teams;
mod tournaments;

pub fn config(cfg: &mut ServiceConfig) {
    mappools::config(cfg);
    matches::config(cfg);
    players::config(cfg);
    teams::config(cfg);
    tournaments::config(cfg);
}
