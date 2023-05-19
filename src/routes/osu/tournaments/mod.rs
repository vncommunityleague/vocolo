use axum::routing::get;
use axum::{Router, Json};
use axum::extract::{State, Path};
use axum::http::StatusCode;
use mongodb::bson::{doc};
use serde::{Deserialize, Serialize};

use crate::models::osu::tournaments::OsuTournament;
use crate::repository::Repo;
use crate::routes::{ApiError, ApiResult, get_option_from_query, ApiResponse};

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
        // .nest(":tournament_id/staff", staff::init_routes())
        // .nest(":tournament_id/teams", teams::init_routes())
}

pub async fn tournaments_get(
    State(repo): State<Repo>, 
    Path(tournament_id): Path<String>
) -> ApiResult<OsuTournament> {
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    let tournament = match get_option_from_query(tournament) {
        Some(value) => value,
        None => return Err(ApiError::TournamentNotFound),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(tournament)
    )
}

// TODO: Add SearchConfig
pub async fn tournaments_list(State(repo): State<Repo>) -> ApiResult<Vec<OsuTournament>> {
    let tournaments = repo.osu.tournaments.list_tournaments().await;

    // if tournaments.is_err() {
    //     return Err(ApiError::from_repo_error(tournaments.err().unwrap()));
    // }

    // let tournaments = tournaments.unwrap();

    // if tournaments.is_none() {
    //     return Err(ApiError::TournamentNotFound);
    // }

    // Ok(HttpResponse::Ok().json(tournaments.unwrap()))

    Ok(None)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentCreateRequest {
    pub title: String,
    pub slug: String,
}

pub async fn tournaments_create(
    State(repo): State<Repo>, 
    Json(data): Json<TournamentCreateRequest>,
) -> ApiResult<OsuTournament> {
    let tournament_id = repo
        .osu
        .tournaments
        .create_tournament(data.slug.clone(), data.title.clone())
        .await;

    let tournament_id = match get_option_from_query(tournament_id) {
        Some(value) => value,
        None => return Err(ApiError::InternalServerError { message: "Unkown".to_owned() }),
    };

    Ok(tournament_id)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentEditRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
}

pub async fn tournaments_modify(
    State(repo): State<Repo>,
    Path(tournament_id): Path<String>,
    Json(data): Json<TournamentEditRequest>,
) -> ApiResult<OsuTournament> {
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    let tournament = match get_option_from_query(tournament) {
        Some(value) => value,
        None => return Err(ApiError::TournamentNotFound),
    };

    if let Some(t) = &data.title {
        tournament.info.title = t.to_string();
    }
   
    if let Some(s) = &data.slug {
        tournament.info.slug = s.to_string();
    }

    repo.osu
        .tournaments
        .replace_tournament(&tournament_id, tournament)
        .await
        .unwrap();

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(tournament)
    )
}

pub async fn tournaments_delete(
    State(repo): State<Repo>, 
    Path(tournament_id): Path<String>
) -> ApiResult<()> {
    let tournament = repo
        .osu
        .tournaments
        .delete_tournament(&tournament_id)
        .await;

    match &get_option_from_query(tournament) {
        Some(value) => tournament = value,
        None => Err(ApiError::TournamentNotFound),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::NO_CONTENT)
        // .body(tournament)
    )
}
