use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::models::osu::tournaments::OsuTournament;
use crate::repository::Repo;
use crate::routes::{convert_result, ApiResponse, ApiResult};

mod mappools;
mod matches;
mod players;
mod staff;
// mod stages;
// mod teams;

pub fn init_routes() -> Router<Repo> {
    Router::new()
        .route("/", get(tournaments_list).post(tournaments_create))
        .route(
            "/:tournament_id",
            get(tournaments_get)
                .patch(tournaments_modify)
                .delete(tournaments_delete),
        )
        .nest("/mappools", mappools::init_routes())
        .nest("/matches", matches::init_routes())
        .nest("/:tournament_id/players", players::init_routes())
        .nest("/:tournament_id/staff", staff::init_routes())
    // .nest(":tournament_id/teams", teams::init_routes())
}

pub async fn tournaments_get(
    State(repo): State<Repo>,
    Path(tournament_id): Path<String>,
) -> ApiResult<OsuTournament> {
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    let tournament = match convert_result(tournament, "tournament") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(tournament))
}

// TODO: Add SearchConfig
pub async fn tournaments_list(State(repo): State<Repo>) -> ApiResult<Vec<OsuTournament>> {
    let tournaments = repo.osu.tournaments.list_tournaments().await;

    let tournaments = match convert_result(tournaments, "tournaments") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    if tournaments.is_empty() {
        return Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT));
    }

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(tournaments))
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
    todo!()
    // let tournament = repo
    //     .osu
    //     .tournaments
    //     .create_tournament(data.slug.clone(), data.title.clone())
    //     .await;
    //
    // let tournament = match convert_result(tournament, "tournament") {
    //     Ok(value) => value,
    //     Err(e) => return Err(e),
    // };
    //
    // Ok(ApiResponse::new()
    //     .status_code(StatusCode::CREATED)
    //     .body(tournament))
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

    let mut tournament = match convert_result(tournament, "tournament") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    if let Some(t) = &data.title {
        tournament.info.title = t.to_string();
    }

    if let Some(s) = &data.slug {
        tournament.info.slug = s.to_string();
    }

    let tournament = repo
        .osu
        .tournaments
        .replace_tournament(&tournament_id, tournament)
        .await
        .unwrap()
        .unwrap();

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(tournament))
}

pub async fn tournaments_delete(
    State(repo): State<Repo>,
    Path(tournament_id): Path<String>,
) -> ApiResult<()> {
    let tournament = repo
        .osu
        .tournaments
        .delete_match_by_id(&tournament_id)
        .await;

    let tournament = match convert_result(tournament, "tournament") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}
