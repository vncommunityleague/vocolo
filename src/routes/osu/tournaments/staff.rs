use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::models::tournaments::TournamentStaff;
use crate::models::user::Role;
use crate::repository::Repo;
use crate::routes::{convert_result, ApiError, ApiResponse, ApiResult};

pub fn init_routes() -> Router<Repo> {
    Router::new()
        .route("/", post(staff_add))
        .route(
            "/:user_id",
            put(staff_modify).delete(staff_delete),
        )
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddStaffRequest {
    pub id: String,
    pub roles: Option<Role>,
}

pub async fn staff_add(
    State(repo): State<Repo>,
    Path(tournament_id): Path<String>,
    Json(data): Json<AddStaffRequest>,
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

    let user = repo.user.find_user_by_osu_id(&data.id).await;

    if user.is_none() {
        return Err(ApiError::NotFound("user".to_string()));
    }

    let user = user.unwrap();
    let roles = data.roles.clone().unwrap_or_default();

    tournament.info.staff.push(TournamentStaff {
        id: user.osu_id,
        roles: vec![roles],
    });

    let _ = repo
        .osu
        .tournaments
        .replace_tournament(&tournament_id, tournament)
        .await;

    Ok(ApiResponse::new().status_code(StatusCode::OK))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditStaffRequest {
    pub roles: Option<Role>,
}

pub async fn staff_modify(
    State(repo): State<Repo>,
    Path((tournament_id, user_id)): Path<(String, String)>,
    Json(data): Json<EditStaffRequest>,
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
    let staff = repo.user.find_user_by_osu_id(&user_id).await;

    if staff.is_none() {
        return Err(ApiError::NotFound("staff".to_string()));
    }

    let staff = staff.unwrap();
    let roles = data.roles.unwrap_or_default();

    for s in tournament.info.staff.iter_mut() {
        if s.id == staff.osu_id {
            s.roles.push(roles);
        }
    }

    let _ = repo
        .osu
        .tournaments
        .replace_tournament(&tournament_id, tournament)
        .await;

    Ok(ApiResponse::new().status_code(StatusCode::OK))
}

pub async fn staff_delete(
    State(repo): State<Repo>,
    Path((tournament_id, user_id)): Path<(String, String)>,
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
    let staff = tournament.info.staff.clone();
    let mut done = false;

    for (i, s) in staff.iter().enumerate() {
        if s.id == *user_id {
            tournament.info.staff.remove(i);
            done = true;
        }
    }

    if !done {
        return Err(ApiError::NotFound("staff".to_string()));
    }

    let _ = repo
        .osu
        .tournaments
        .replace_tournament(&tournament_id, tournament)
        .await;

    Ok(ApiResponse::new().status_code(StatusCode::OK))
}
