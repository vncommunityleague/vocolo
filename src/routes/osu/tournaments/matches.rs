use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::models::osu::tournaments::{OsuMappool, OsuMatch, OsuTeam};
use crate::models::tournaments::MatchInfo;
use crate::repository::{to_object_id, Repo};
use crate::routes::{convert_result, ApiError, ApiResponse, ApiResult};

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
/// It contains all the information about an osu!match and limit some.
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
    let mappool = match original.mappool {
        Some(ref id) => {
            let mappool = repo.osu.tournaments.find_mappool(doc! { "_id": id }).await;

            match convert_result(mappool, "mappool") {
                Ok(value) => value,
                Err(e) => return Err(e),
            }
        }
        None => return Err(ApiError::InternalServerError),
    };

    let tournament = match original.info.tournament {
        Some(ref id) => {
            let tournament = repo
                .osu
                .tournaments
                .find_tournament(doc! { "_id": id })
                .await;

            match convert_result(tournament, "tournament") {
                Ok(value) => value,
                Err(e) => return Err(e),
            }
        }
        None => return Err(ApiError::InternalServerError),
    };

    Ok(PublicOsuMatch {
        info: original.info,
        mappool,
        blue_team: tournament
            .get_team(to_object_id(original.blue_team.unwrap().as_ref()))
            .await
            .unwrap()
            .1,
        red_team: tournament
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
    let osu_match = repo.osu.tournaments.find_match_by_id(&match_id).await;
    let osu_match = match convert_result(osu_match, "match") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };
    let osu_match = match to_public(repo, osu_match).await {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(osu_match))
}

pub async fn matches_list(State(repo): State<Repo>) -> ApiResult<Vec<PublicOsuMatch>> {
    let osu_matches = repo.osu.tournaments.list_matches().await;

    let osu_matches = match convert_result(osu_matches, "matches") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

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

    let osu_match = repo.osu.tournaments.create_match(osu_match).await;

    let osu_match = match convert_result(osu_match, "match") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    let osu_match = match to_public(repo, osu_match).await {
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
    let osu_match = repo
        .osu
        .tournaments
        .update_match_by_id(&match_id, bson::to_document(&data).unwrap())
        .await;

    let osu_match = match convert_result(osu_match, "match") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(osu_match))
}

pub async fn matches_delete(
    State(repo): State<Repo>,
    Path(match_id): Path<String>,
) -> ApiResult<PublicOsuMatch> {
    let osu_match = repo.osu.tournaments.delete_match_by_id(&match_id).await;

    let osu_match = match convert_result(osu_match, "match") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}
