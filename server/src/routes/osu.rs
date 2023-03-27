use actix_web::web;
use actix_web::web::ServiceConfig;
use actix_web::{delete, error::HttpError, get, patch, post, HttpResponse};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("mappools")
            .service(mappools_get)
            .service(mappools_list)
            .service(mappools_post)
            .service(mappools_patch)
            .service(mappools_delete),
    );

    cfg.service(
        web::scope("matches")
            .service(matches_get)
            .service(matches_list)
            .service(matches_post)
            .service(matches_patch)
            .service(matches_delete),
    );

    cfg.service(
        web::scope("players")
            .service(players_get)
            .service(players_list)
            .service(players_post)
            .service(players_patch)
            .service(players_delete),
    );

    cfg.service(
        web::scope("tournaments")
            .service(tournaments_get)
            .service(tournaments_list)
            .service(tournaments_post)
            .service(tournaments_patch)
            .service(tournaments_delete)
            .service(tournaments_teams_list),
    );
}

// MAPPOOLS

#[get("{id}")]
pub async fn mappools_get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("")]
pub async fn mappools_list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn mappools_post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn mappools_patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn mappools_delete() -> Result<HttpResponse, HttpError> {
    todo!();
}

// MATCHES

#[get("{id}")]
pub async fn matches_get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("")]
pub async fn matches_list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn matches_post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn matches_patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn matches_delete() -> Result<HttpResponse, HttpError> {
    todo!();
}

// PLAYERS

#[get("{id}")]
pub async fn players_get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("")]
pub async fn players_list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn players_post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn players_patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn players_delete() -> Result<HttpResponse, HttpError> {
    todo!();
}

// TEAMS

#[get("{id}")]
pub async fn teams_get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("")]
pub async fn teams_list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn teams_post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn teams_patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn teams_delete() -> Result<HttpResponse, HttpError> {
    todo!();
}

// TOURNAMENTS

#[get("{id}")]
pub async fn tournaments_get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("")]
pub async fn tournaments_list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn tournaments_post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn tournaments_patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn tournaments_delete() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("{id}/teams")]
pub async fn tournaments_teams_list() -> Result<HttpResponse, HttpError> {
    todo!();
}
