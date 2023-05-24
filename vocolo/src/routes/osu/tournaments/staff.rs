use crate::models::osu::tournaments::OsuTournament;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use bson::doc;
use serde::{Deserialize, Serialize};

use crate::models::user::{Role, User};
use crate::repository::model::ModelExt;
use crate::repository::{to_object_id, Repo};
use crate::routes::{ApiError, ApiResponse, ApiResult};

pub fn init_routes() -> Router<Repo> {
    Router::new()
        .route("/", post(staff_add))
        .route("/:user_id", put(staff_modify).delete(staff_delete))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddStaffRequest {
    pub id: String,
    pub roles: Vec<String>,
}

pub async fn staff_add(
    State(repo): State<Repo>,
    Path(tournament_id): Path<String>,
    Json(data): Json<AddStaffRequest>,
) -> ApiResult<()> {
    let user = User::find_by_id(repo.user.user_col, &to_object_id(&data.id))
        .await
        .map_err(ApiError::Database)?;

    if user.is_none() {
        return Err(ApiError::NotFound("user".to_string()));
    }

    let tournament = OsuTournament::find_one_and_update(
        repo.osu.tournaments.tournaments_col,
        doc! {
            "$or": [
                {
                    "_id": to_object_id(&tournament_id)
                },
                {
                    "slug": tournament_id
                }
            ]
        },
        doc! {
            "$push": {
                "info.staff": {
                    "id": data.id,
                    "roles": data.roles
                }
            }
        },
    )
    .await
    .map_err(ApiError::Database)?;

    Ok(ApiResponse::new().status_code(StatusCode::OK))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditStaffRequest {
    pub roles: Option<Vec<String>>,
}

pub async fn staff_modify(
    State(repo): State<Repo>,
    Path((tournament_id, user_id)): Path<(String, String)>,
    Json(data): Json<EditStaffRequest>,
) -> ApiResult<()> {
    let user = User::find_by_id(repo.user.user_col, &to_object_id(&user_id))
        .await
        .map_err(ApiError::Database)?;

    if user.is_none() {
        return Err(ApiError::NotFound("user".to_string()));
    }

    let osu_tournament = OsuTournament::find_one_and_update(
        repo.osu.tournaments.tournaments_col,
        doc! {
            "$or": [
                {
                    "_id": to_object_id(&tournament_id)
                },
                {
                    "slug": tournament_id
                }
            ],
            "info.staff.id": user_id
        },
        doc! {
            "$set": {
                "info.staff.$.roles": data.roles.unwrap_or_default()
            }
        },
    );

    Ok(ApiResponse::new().status_code(StatusCode::OK))
}

pub async fn staff_delete(
    State(repo): State<Repo>,
    Path((tournament_id, user_id)): Path<(String, String)>,
) -> ApiResult<()> {
    let user = User::find_by_id(repo.user.user_col, &to_object_id(&user_id))
        .await
        .map_err(ApiError::Database)?;

    if user.is_none() {
        return Err(ApiError::NotFound("user".to_string()));
    }

    let tournament = OsuTournament::find_one_and_update(
        repo.osu.tournaments.tournaments_col,
        doc! {
            "$or": [
                {
                    "_id": to_object_id(&tournament_id)
                },
                {
                    "slug": tournament_id
                }
            ]
        },
        doc! {
            "$pull": {
                "info.staff": {
                    "id": user_id
                }
            }
        },
    );

    Ok(ApiResponse::new().status_code(StatusCode::OK))
}
