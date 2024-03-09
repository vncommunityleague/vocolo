use axum::{Json, Router};
use axum::extract::State;
use axum::routing::post;
use axum_garde::WithValidation;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use vocolo_entity::osu_team::ActiveModel;

use crate::error::Error;
use crate::models::osu::APIOsuTeam;
use crate::routes::AppState;
use crate::util::auth::SessionUser;

pub fn routes() -> Router<AppState> {
    Router::new().route("/:tournament_id/register", post(register))
}

#[derive(garde::Validate, Serialize, Deserialize, Debug)]
#[garde(allow_unvalidated)]
pub struct OsuTeamCreation {
    pub name: String,
}

pub async fn register(
    SessionUser(user): SessionUser,
    State(state): State<AppState>,
    WithValidation(data): WithValidation<Json<OsuTeamCreation>>,
) -> Result<Json<APIOsuTeam>, Error> {
    let data = data.into_inner();

    let team = ActiveModel {
        name: Set(data.name),
        ..Default::default()
    };

    todo!()
}
