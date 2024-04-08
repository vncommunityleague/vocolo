use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use axum_garde::WithValidation;

use vocolo_database::{osu::Tournament, osu::TournamentTeam, Database};
use vocolo_internal::{Result, UserConnections};
use vocolo_models::v1;

use crate::routes::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/teams", get(tournament_team_list))
        .route("/players", get(tournament_player_list))
        .route("/register", post(tournament_register))
}

pub async fn tournament_team_list(
    State(db): State<Database>,
    Path(tournament_id): Path<String>,
) -> Result<Json<Vec<v1::osu::TournamentTeam>>> {
    let teams = Tournament::get_teams(&db, &tournament_id).await?;
    let teams: Vec<v1::osu::TournamentTeam> = teams.into_iter().map(|v| v.into()).collect();

    Ok(Json(teams))
}

pub async fn tournament_player_list(
    State(db): State<Database>,
    Path(tournament_id): Path<String>,
) -> Result<Json<v1::osu::ListPlayerResponse>> {
    let teams = Tournament::get_teams(&db, &tournament_id).await?;
    let players = teams.into_iter().flat_map(|t| t.players).collect();

    Ok(Json(v1::osu::ListPlayerResponse { players }))
}

pub async fn tournament_register(
    State(db): State<Database>,
    Path(tournament_id): Path<String>,
    captain_connections: UserConnections,
    WithValidation(data): WithValidation<Json<v1::osu::RegisterTournamentTeamRequest>>,
) -> Result<Json<v1::osu::RegisterTournamentTeamResponse>> {
    let data = data.into_inner();
    let captain_id = captain_connections.osu.id;

    let mut team: TournamentTeam = data.into();
    team.captain = captain_id;
    team.players.push(captain_id);

    let team = Tournament::register_team(&db, &tournament_id, team).await?;

    Ok(Json(v1::osu::RegisterTournamentTeamResponse {
        id: team.id,
    }))
}
