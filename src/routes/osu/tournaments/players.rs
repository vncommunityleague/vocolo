use actix_web::web::{Data, ServiceConfig};
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::repository::Repo;
use crate::routes::{ApiError, ApiResult};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(players_tournament_get);
    cfg.service(players_team_get);
    cfg.service(players_team_add);
}

#[get("{tournament_id}/players")]
pub async fn players_tournament_get(
    repo: Data<Repo>,
    info: web::Path<(String,)>,
) -> ApiResult {
    let tournament_id = info.into_inner().0;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    if tournament.is_err() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = tournament.unwrap();

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let mut players = Vec::new();

    for player in tournament.unwrap().players().await {
        players.push(repo.user.find_user_by_osu_id(&player).await.unwrap());
    }

    Ok(HttpResponse::Ok().json(players))
}

#[get("{tournament_id}/teams/{team_id}/players")]
pub async fn players_team_get(
    repo: Data<Repo>,
    info: web::Path<(String, String)>,
) -> ApiResult {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let team_id = &path.1;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_err() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = tournament.unwrap();

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = tournament.unwrap();
    let team = tournament.get_team(team_id.to_string()).await;

    if team.is_none() {
        return Err(ApiError::TournamentTeamNotFound);
    }

    Ok(HttpResponse::Ok().json(team.unwrap().info.players))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamJoinRequest {
    osu_id: u64,
}

#[post("{tournament_id}/teams/{team_id}/players")]
pub async fn players_team_add(
    repo: Data<Repo>,
    info: web::Path<(String, String)>,
    data: web::Json<TeamJoinRequest>,
) -> ApiResult {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let team_id = &path.1;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_err() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = tournament.unwrap();

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = &mut tournament.unwrap();
    let team_pos = tournament.get_team_position(team_id.to_string()).await;

    if team_pos.is_none() {
        return Err(ApiError::TournamentTeamNotFound);
    }

    tournament.teams[team_pos.unwrap()]
        .info
        .players
        .push(data.osu_id.to_string());

    repo.osu
        .tournaments
        .replace_tournament(tournament_id, tournament.clone())
        .await;

    Ok(HttpResponse::NoContent().finish())
}
