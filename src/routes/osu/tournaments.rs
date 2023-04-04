use crate::repository::Repo;
use actix_web::error::HttpError;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use thiserror::Error;

pub fn config(cfg: &mut ServiceConfig) {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentCreationData {
    pub title: String,
    pub slug: String,
}

#[derive(Error, Debug)]
enum TournamentError {}

#[get("{id}")]
pub async fn tournaments_get(
    repo: Data<Repo>,
    tournament: web::Path<(String,)>,
) -> Result<HttpResponse, HttpError> {
    let tournament = repo
        .osu
        .find_tournament_by_slug(tournament.into_inner().0)
        .await;

    if tournament.is_some() {
        Ok(HttpResponse::Ok().json(tournament.unwrap()))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("")]
pub async fn tournaments_list(repo: Data<Repo>) -> Result<HttpResponse, HttpError> {
    let tournaments = repo.osu.find_tournaments().await;

    if tournaments.is_some() {
        Ok(HttpResponse::Ok().json(tournaments.unwrap()))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[post("")]
pub async fn tournaments_post(
    repo: Data<Repo>,
    data: web::Json<TournamentCreationData>,
) -> Result<HttpResponse, HttpError> {
    let tournament_id = repo
        .osu
        .create_tournament(data.slug.clone(), data.title.clone())
        .await;
    Ok(HttpResponse::Ok().json(tournament_id.unwrap()))
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
pub async fn tournaments_teams_list(
    repo: Data<Repo>,
    tournament: web::Path<(String,)>,
) -> Result<HttpResponse, HttpError> {
    let tournament = repo
        .osu
        .find_tournament_by_slug(tournament.into_inner().0)
        .await;

    if tournament.is_some() {
        Ok(HttpResponse::Ok().json(tournament.unwrap().teams))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
