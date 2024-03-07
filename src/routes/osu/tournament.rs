use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_garde::WithValidation;
use chrono::Utc;
use sea_orm::prelude::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};

use vocolo_entity::osu_tournament::{ActiveModel, Column, Model};
use vocolo_entity::prelude::OsuTournament;

use crate::error::Error;
use crate::models::osu::APIOsuTournament;
use crate::models::user::APIUser;
use crate::routes::AppState;
use crate::util::auth::SessionUser;

const MIN_SLUG_LENGTH: usize = 2;
const MAX_SLUG_LENGTH: usize = 8;
const MIN_NAME_LENGTH: usize = 4;
const MAX_NAME_LENGTH: usize = 64;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(tournament_create)).route(
        "/:id",
        get(tournament_get)
            .patch(tournament_edit)
            .delete(tournament_delete),
    )
}

#[derive(garde::Validate, Serialize, Deserialize)]
#[garde(allow_unvalidated)]
pub struct OsuTournamentCreation {
    #[garde(length(min = MIN_SLUG_LENGTH, max = MAX_SLUG_LENGTH))]
    pub slug: String,
    #[garde(length(min = MIN_NAME_LENGTH, max = MAX_NAME_LENGTH))]
    pub name: String,
    pub start_date: Option<DateTime>,
    pub end_date: Option<DateTime>,
    pub registration_start_date: Option<DateTime>,
    pub registration_end_date: Option<DateTime>,
}

pub async fn tournament_create(
    State(state): State<AppState>,
    SessionUser(user): SessionUser,
    WithValidation(data): WithValidation<Json<OsuTournamentCreation>>,
) -> Result<Json<APIOsuTournament>, Error> {
    let data = data.into_inner();
    
    let tournament = ActiveModel {
        slug: Set(data.slug),
        name: Set(data.name),
        start_date: Set(data.start_date.unwrap_or(Utc::now().naive_utc())),
        end_date: Set(data.end_date.unwrap_or(Utc::now().naive_utc())),
        registration_start_date: Set(data
            .registration_start_date
            .unwrap_or(Utc::now().naive_utc())),
        registration_end_date: Set(data.registration_end_date.unwrap_or(Utc::now().naive_utc())),
        ..Default::default()
    };

    let tournament: Model = tournament.insert(&state.database).await?;

    Ok(Json(APIOsuTournament::from(tournament)))
}

pub async fn tournament_get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<APIOsuTournament>, Error> {
    let tournament = if let Ok(id) = id.parse::<i32>() {
        OsuTournament::find_by_id(id)
    } else {
        OsuTournament::find().filter(Column::Slug.eq(id))
    }
    .one(&state.database)
    .await
    .map_err(Error::Database)?
    .ok_or(Error::NotFound("osu_tournament".to_owned()))?;

    Ok(Json(APIOsuTournament::from(tournament)))
}

#[derive(garde::Validate, Serialize, Deserialize)]
#[garde(allow_unvalidated)]
pub struct OsuTournamentUpdate {
    #[garde(length(min = MIN_SLUG_LENGTH, max = MAX_SLUG_LENGTH))]
    pub slug: Option<String>,
    #[garde(length(min = MIN_NAME_LENGTH, max = MAX_NAME_LENGTH))]
    pub name: Option<String>,
    pub start_date: Option<DateTime>,
    pub end_date: Option<DateTime>,
    pub registration_start_date: Option<DateTime>,
    pub registration_end_date: Option<DateTime>,
}

pub async fn tournament_edit(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    WithValidation(data): WithValidation<Json<OsuTournamentUpdate>>,
) -> Result<Json<APIOsuTournament>, Error> {
    let data = data.into_inner();

    let tournament = OsuTournament::find_by_id(id)
        .one(&db)
        .await
        .map_err(Error::Database)?
        .ok_or(Error::NotFound("osu_tournament".to_owned()))?;

    let tournament = ActiveModel {
        id: Set(tournament.id),
        slug: Set(data.slug.unwrap_or(tournament.slug)),
        name: Set(data.name.unwrap_or(tournament.name)),
        start_date: Set(data.start_date.unwrap_or(tournament.start_date)),
        end_date: Set(data.end_date.unwrap_or(tournament.end_date)),
        registration_start_date: Set(data
            .registration_start_date
            .unwrap_or(tournament.registration_start_date)),
        registration_end_date: Set(data
            .registration_end_date
            .unwrap_or(tournament.registration_end_date)),
    }
    .update(&db)
    .await
    .map_err(Error::Database)?;

    Ok(Json(APIOsuTournament::from(tournament)))
}

pub async fn tournament_delete(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<StatusCode, Error> {
    let tournament = OsuTournament::find_by_id(id)
        .one(&db)
        .await
        .map_err(Error::Database)?
        .ok_or(Error::NotFound("osu_tournament".to_owned()))?;

    tournament.delete(&db).await.map_err(Error::Database)?;

    Ok(StatusCode::NO_CONTENT)
}
