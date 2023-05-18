use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::repository::Repo;
use crate::routes::{ApiError, ApiResult};

mod mappools;
mod matches;
mod players;
mod staff;
mod stages;
mod teams;

pub fn init_routes() -> Router {
    Router::new()
        .route(":tournament_id", get(tournaments_get).patch(tournaments_modify).delete(tournaments_delete))
        .route("", get(tournaments_list).post(tournaments_create))
        .nest(":tournament_id/mappools", mappools::init_routes())
        .nest(":tournament_id/players", players::init_routes())
        .nest(":tournament_id/staff", staff::init_routes())
        .nest(":tournament_id/teams", teams::init_routes())
}

#[derive(Serialize, Deserialize, Clone)]
struct OsuTournamentResponse {
    id: ObjectId,
    title: String,
    slug: String,
}

pub async fn tournaments_get(repo: Data<Repo>, Path(tournament_id): Path<String>) -> ApiResult {
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_err() {
        return Err(ApiError::from_repo_error(tournament.err().unwrap()));
    }

    let tournament = tournament.unwrap();

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    Ok(Json(tournament.unwrap()))
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
    Json(data): Json<TournamentCreateRequest>,
) -> ApiResult {
    let tournament_id = repo
        .osu
        .tournaments
        .create_tournament(data.slug.clone(), data.title.clone())
        .await;

    match tournament_id 

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
    let id_or_slug = info.into_inner().0;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&id_or_slug)
        .await
        .unwrap();

    if tournament.is_some() {
        let mut tournament = tournament.unwrap();
        if let Some(t) = &data.title {
            tournament.info.title = t.to_string();
        }
        if let Some(s) = &data.slug {
            tournament.info.slug = s.to_string();
        }
        repo.osu
            .tournaments
            .replace_tournament(&id_or_slug, tournament)
            .await
            .unwrap();
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn tournaments_delete(repo: Data<Repo>, Path(tournament_id): Path<String>) -> ApiResult {
    let tournament = repo.osu.tournaments.delete_tournament(tournament_id).await;

    if tournament.is_err() {
        return Err(ApiError::from_repo_error(tournament.err().unwrap()));
    }

    let tournament = tournament.unwrap();

    match tournament {
        Ok(tournament) => CustomResponseBuilder::new()
            .status_code(StatusCode::NO_CONTENT)
            .build(),
        Err(e) => Err(ApiError::TournamentNotFound)
    }
}
