use crate::models::osu::tournaments::OsuTeam;
use crate::models::tournaments::TournamentTeamInfo;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{delete, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::repository::Repo;
use crate::routes::ApiError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(teams_create);
    cfg.service(teams_modify);
    cfg.service(teams_delete);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamCreateRequest {
    pub name: String,
    pub avatar_url: Option<String>,
}

#[post("{tournament_id}/teams")]
pub async fn teams_create(
    repo: Data<Repo>,
    info: web::Path<(String,)>,
    data: web::Json<TeamCreateRequest>,
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let tournament_id = path.0;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let mut tournament = tournament.unwrap();

    let mut team = OsuTeam {
        info: TournamentTeamInfo {
            name: data.name.clone(),
            captain: "".to_string(),
            avatar_url: None,

            players: vec![],
        },
    };

    if data.avatar_url.is_some() {
        team.info.avatar_url = Some(data.avatar_url.clone().unwrap());
    }

    tournament.teams.push(team.clone());

    let new_tournament = repo
        .osu
        .tournaments
        .replace_tournament(&tournament_id, tournament)
        .await;

    if new_tournament.is_some() {
        Ok(HttpResponse::Ok().json(new_tournament.unwrap()))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[patch("{tournament_id}/teams/{team_id}")]
pub async fn teams_modify(
    repo: Data<Repo>,
    info: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let tournament_id = &*path.0;
    let team_id = path.1;

    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let team = tournament.unwrap().get_team(team_id).await;

    if team.is_none() {
        return Err(ApiError::TeamNotFound);
    }

    Ok(HttpResponse::Ok().json(team))
}

#[delete("{tournament_id}/teams/{team_id}")]
pub async fn teams_delete(
    repo: Data<Repo>,
    info: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let tournament_id = &*path.0;
    let team_id = path.1;

    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let mut tournament = tournament.unwrap();
    let index = tournament
        .teams
        .iter()
        .position(|x| x.info.name.eq(&team_id));

    if index.is_none() {
        return Err(ApiError::TeamNotFound);
    }

    tournament.teams.remove(index.unwrap());
    repo.osu
        .tournaments
        .replace_tournament(tournament_id, tournament)
        .await;

    Ok(HttpResponse::NoContent().finish())
}
