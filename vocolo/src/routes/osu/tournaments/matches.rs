use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::models::osu::tournaments::{OsuMappool, OsuMatch, OsuTeam, OsuTournament};
use crate::models::tournaments::MatchInfo;
use crate::repository::model::ModelExt;
use crate::repository::{to_object_id, Repo};
use crate::routes::{ApiError, ApiResponse, ApiResult};

pub fn init_routes() -> Router<Repo> {
    Router::new()
        .route("/", get(matches_list).post(matches_create))
        .route(
            "/:match_id",
            get(matches_get)
                .patch(matches_update)
                .delete(matches_delete),
        )
}

/// This model is used only for REST APIs response
/// It contains all the information about an osu_old!match and limit some.
/// However, DO NOT USE THIS MODEL FOR ANYTHING ELSE, ESPECIALLY NOT FOR DATABASE
#[derive(Serialize, Deserialize, Clone)]
pub struct PublicOsuMatch {
    #[serde(flatten)]
    pub info: MatchInfo,

    pub mappool: OsuMappool,

    pub blue_team: OsuTeam,
    pub red_team: OsuTeam,

    pub osu_match_id: i64,
}

async fn to_public(repo: Repo, original: OsuMatch) -> Result<PublicOsuMatch, ApiError> {
    let osu_mappool = match original.mappool {
        Some(ref id) => {
            let osu_mappool =
                OsuMappool::find_by_id(repo.osu.tournaments.mappools_col, &to_object_id(&id))
                    .await
                    .map_err(ApiError::Database)?;

            match osu_mappool {
                Some(value) => value,
                None => return Err(ApiError::NotFound("osu_mappool".to_string())),
            }
        }
        None => return Err(ApiError::InternalServerError),
    };

    let osu_tournament = match original.info.tournament {
        Some(ref id) => {
            let osu_tournament =
                OsuTournament::find_by_id(repo.osu.tournaments.tournaments_col, id)
                    .await
                    .map_err(ApiError::Database)?;

            match osu_tournament {
                Some(value) => value,
                None => return Err(ApiError::NotFound("osu_tournament".to_string())),
            }
        }
        None => return Err(ApiError::InternalServerError),
    };

    Ok(PublicOsuMatch {
        info: original.info,
        mappool: osu_mappool,
        blue_team: osu_tournament
            .get_team(to_object_id(original.blue_team.unwrap().as_ref()))
            .await
            .unwrap()
            .1,
        red_team: osu_tournament
            .get_team(to_object_id(original.red_team.unwrap().as_ref()))
            .await
            .unwrap()
            .1,
        osu_match_id: original.osu_match_id,
    })
}

pub async fn matches_get(
    State(repo): State<Repo>,
    Path(match_id): Path<String>,
) -> ApiResult<PublicOsuMatch> {
    let osu_match = OsuMatch::find_by_id(
        repo.clone().osu.tournaments.matches_col,
        &to_object_id(&match_id),
    )
    .await
    .map_err(ApiError::Database)?;

    let osu_match = match osu_match {
        Some(value) => value,
        None => return Err(ApiError::NotFound("osu_match".to_string())),
    };

    let osu_match = match to_public(repo.clone(), osu_match).await {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(osu_match))
}

pub async fn matches_list(State(repo): State<Repo>) -> ApiResult<Vec<PublicOsuMatch>> {
    let osu_matches = OsuMatch::list(repo.clone().osu.tournaments.matches_col)
        .await
        .map_err(ApiError::Database)?;

    if osu_matches.is_empty() {
        return Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT));
    }

    let mut result = Vec::new();

    for osu_match in osu_matches {
        let osu_match = match to_public(repo.clone(), osu_match).await {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        result.push(osu_match);
    }

    Ok(ApiResponse::new().status_code(StatusCode::OK).body(result))
}

#[derive(Deserialize)]
pub struct MatchCreationData {
    pub title: String,

    pub mappool_id: Option<String>,
}

pub async fn matches_create(
    State(repo): State<Repo>,
    Json(data): Json<MatchCreationData>,
) -> ApiResult<PublicOsuMatch> {
    let mut osu_match = OsuMatch::default();
    osu_match.info.title = data.title;

    let osu_match = OsuMatch::create(repo.clone().osu.tournaments.matches_col, osu_match)
        .await
        .map_err(ApiError::Database)?;

    let osu_match = match to_public(repo.clone(), osu_match).await {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::CREATED)
        .body(osu_match))
}

#[derive(Serialize, Deserialize)]
pub struct MatchUpdateData {
    pub title: Option<String>,

    pub mappool: Option<String>,
}

pub async fn matches_update(
    State(repo): State<Repo>,
    Path(match_id): Path<String>,
    Json(data): Json<MatchUpdateData>,
) -> ApiResult<OsuMatch> {
    let osu_match = OsuMatch::find_one_and_update(
        repo.osu.tournaments.matches_col,
        doc! {
            "_id": to_object_id(&match_id)
        },
        doc! {
            "$set": bson::to_document(&data).unwrap()
        },
    )
    .await
    .map_err(ApiError::Database)?;

    let osu_match = match osu_match {
        Some(value) => value,
        None => return Err(ApiError::NotFound("osu_match".to_string())),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(osu_match))
}

pub async fn matches_delete(
    State(repo): State<Repo>,
    Path(match_id): Path<String>,
) -> ApiResult<()> {
    let _ = OsuMatch::delete_one(
        repo.osu.tournaments.matches_col,
        doc! {
            "_id": to_object_id(&match_id)
        },
    )
    .await
    .map_err(ApiError::Database)?;

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}
