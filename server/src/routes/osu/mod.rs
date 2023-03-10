use actix_web::web;

pub mod mappools;
pub mod matches;
pub mod players;
pub mod teams;
pub mod tournaments;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("osu")
            .configure(init_mappool_routes)
            .configure(init_match_routes)
            .configure(init_player_routes)
            .configure(init_team_routes)
            .configure(init_tournament_routes),
    );
}

fn init_mappool_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(mappools::list);
    cfg.service(
        web::scope("mappool")
            .service(mappools::get)
            .service(mappools::post)
            .service(mappools::patch)
            .service(mappools::delete),
    );
}

fn init_match_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(matches::list);
    cfg.service(
        web::scope("match")
            .service(matches::get)
            .service(matches::post)
            .service(matches::patch)
            .service(matches::delete),
    );
}

fn init_player_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(players::list);
    cfg.service(
        web::scope("player")
            .service(players::get)
            .service(players::post)
            .service(players::patch)
            .service(players::delete),
    );
}

fn init_team_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(teams::list);
    cfg.service(
        web::scope("team")
            .service(teams::get)
            .service(teams::post)
            .service(teams::patch)
            .service(teams::delete),
    );
}

fn init_tournament_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(tournaments::list);
    cfg.service(
        web::scope("tournament")
            .service(tournaments::get)
            .service(tournaments::post)
            .service(tournaments::patch)
            .service(tournaments::delete),
    );
}
