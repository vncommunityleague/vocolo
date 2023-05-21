use std::str::FromStr;

use axum::extract::State;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use bson::oid::ObjectId;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::user::User;
use crate::repository::Repo;
use crate::routes::{convert_result, ApiError, ApiResponse, ApiResult};

pub fn init_routes() -> Router<Repo> {
    Router::new()
        .route("/players", get(players_tournament_get))
        .route(
            "/teams/:team_id/players",
            get(players_team_get).post(players_team_add),
        )
}

pub async fn players_tournament_get(
    State(repo): State<Repo>,
    Path(tournament_id): Path<String>,
) -> ApiResult<Vec<User>> {
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    let tournament = match convert_result(tournament, "tournament") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    let mut players = Vec::new();

    for player in tournament.players {
        players.push(repo.user.find_user_by_osu_id(&player).await.unwrap());
    }

    Ok(ApiResponse::new().status_code(StatusCode::OK).body(players))
}

pub async fn players_team_get(
    State(repo): State<Repo>,
    Path((tournament_id, team_id)): Path<(String, String)>,
) -> ApiResult<Vec<String>> {
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    let tournament = match convert_result(tournament, "tournament") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    let team = tournament
        .get_team(ObjectId::from_str(&team_id).unwrap())
        .await;

    if team.is_none() {
        return Err(ApiError::NotFound("team".to_string()));
    }

    let team = team.unwrap().1;

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(team.info.players))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamJoinRequest {
    osu_id: u64,
}

pub async fn players_team_add(
    State(repo): State<Repo>,
    Path((tournament_id, team_id)): Path<(String, String)>,
    Json(data): Json<TeamJoinRequest>,
) -> ApiResult<()> {
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    let mut tournament = match convert_result(tournament, "tournament") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    let team = tournament
        .get_team(ObjectId::from_str(&team_id).unwrap())
        .await;

    if team.is_none() {
        return Err(ApiError::NotFound("team".to_string()));
    }

    let team = team.unwrap();
    let team_pos = team.0;

    tournament.teams[team_pos]
        .info
        .players
        .push(data.osu_id.to_string());

    repo.osu
        .tournaments
        .replace_tournament(&tournament_id, tournament)
        .await;

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}
