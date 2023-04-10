use actix_web::{
    web::{Data, ServiceConfig},
    {delete, get, patch, post, web, HttpResponse},
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::repository::Repo;
use crate::routes::{ApiError, ApiResult};

mod mappools;
mod matches;
mod players;
mod staff;
mod stages;
mod teams;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("tournaments")
            .service(tournaments_get)
            .service(tournaments_list)
            .service(tournaments_create)
            .service(tournaments_modify)
            .service(tournaments_delete)
            .configure(mappools::config)
            .configure(players::config)
            .configure(staff::config)
            .configure(teams::config),
    );
}

#[derive(Serialize, Deserialize, Clone)]
struct OsuTournamentResponse {
    id: ObjectId,
    title: String,
    slug: String,
}

#[get("{tournament_id}")]
pub async fn tournaments_get(repo: Data<Repo>, info: web::Path<(String,)>) -> ApiResult {
    let path = info.into_inner();
    let id_or_slug = &path.0;

    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(id_or_slug)
        .await;

    if tournament.is_err() {
        return Err(ApiError::from_repo_error(tournament.err().unwrap()));
    }

    let tournament = tournament.unwrap();

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    Ok(HttpResponse::Ok().json(tournament.unwrap()))
}

// TODO: Add SearchConfig
#[get("")]
pub async fn tournaments_list(repo: Data<Repo>) -> ApiResult {
    let tournaments = repo.osu.tournaments.list_tournaments().await;

    if tournaments.is_err() {
        return Err(ApiError::from_repo_error(tournaments.err().unwrap()));
    }

    let tournaments = tournaments.unwrap();

    if tournaments.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    Ok(HttpResponse::Ok().json(tournaments.unwrap()))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentCreateRequest {
    pub title: String,
    pub slug: String,
}

#[post("")]
pub async fn tournaments_create(
    repo: Data<Repo>,
    data: web::Json<TournamentCreateRequest>,
) -> ApiResult {
    let tournament_id = repo
        .osu
        .tournaments
        .create_tournament(data.slug.clone(), data.title.clone())
        .await;

    if tournament_id.is_err() {
        return Err(ApiError::from_repo_error(tournament_id.err().unwrap()));
    }

    Ok(HttpResponse::Ok().json(tournament_id.unwrap()))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentEditRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
}

#[patch("{tournament_id}")]
pub async fn tournaments_modify(
    repo: Data<Repo>,
    info: web::Path<(String,)>,
    data: web::Json<TournamentEditRequest>,
) -> ApiResult {
    // let id_or_slug = info.into_inner().0;
    //
    // let tournament = repo.osu.(id_or_slug).await;
    //
    // if tournament.is_some() {
    //     Ok(HttpResponse::NoContent().finish())
    // } else {
    //     Ok(HttpResponse::NotFound().finish())
    // }

    todo!()
}

#[delete("{tournament_id}")]
pub async fn tournaments_delete(repo: Data<Repo>, info: web::Path<(String,)>) -> ApiResult {
    let path = info.into_inner();
    let id_or_slug = &path.0;

    let tournament = repo.osu.tournaments.delete_tournament(id_or_slug).await;

    if tournament.is_err() {
        return Err(ApiError::from_repo_error(tournament.err().unwrap()));
    }

    let tournament = tournament.unwrap();

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    Ok(HttpResponse::Ok().finish())
}
