use crate::routes::ApiError;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{delete, HttpResponse, patch, post, web};
use serde::{Deserialize, Serialize};
use crate::models::tournaments::TournamentStaff;
use crate::models::user::Role;
use crate::repository::Repo;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(staff_add);
    cfg.service(staff_modify);
    cfg.service(staff_delete);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddStaffRequest {
    pub id: String,
    pub roles: Option<Role>
}

#[post("{tournament_id}/staff")]
pub async fn staff_add(
    repo: Data<Repo>,
    info: web::Path<(String,)>,
    data: web::Json<AddStaffRequest>,
) -> Result<HttpResponse, ApiError> {
    let tournament_id = info.into_inner().0;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let mut tournament = tournament.unwrap();
    let user = repo.user.find_user_by_osu_id(&data.id).await;

    if user.is_none() {
        return Err(ApiError::UserNotFound);
    }

    let user = user.unwrap();
    let roles = data.roles.clone().unwrap_or_default();

    tournament.info.staff.push(TournamentStaff {
        id: user.osu_id,
        roles: vec![roles]
    });

    Ok(HttpResponse::Ok().finish())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditStaffRequest {
    pub roles: Option<Role>
}

#[patch("{tournament_id}/staff/{user_id}")]
pub async fn staff_modify(
    repo: Data<Repo>,
    info: web::Path<(String, String)>,
    data: web::Json<EditStaffRequest>,
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let user_id = &path.1;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let mut tournament = tournament.unwrap();
    let staff = repo.user.find_user_by_osu_id(user_id).await;

    if staff.is_none() {
        return Err(ApiError::UserNotFound);
    }

    let staff = staff.unwrap();
    let roles = data.roles.unwrap_or_default();

    for s in tournament.info.staff.iter_mut() {
        if s.id == staff.osu_id {
            s.roles.push(roles);
        }
    }

    Ok(HttpResponse::Ok().finish())
}

#[delete("{tournament_id}/staff/{user_id}")]
pub async fn staff_delete(
    repo: Data<Repo>,
    info: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let user_id = &path.1;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let mut tournament = tournament.unwrap();
    let staff = tournament.info.staff.clone();
    let mut done = false;

    for (i, s) in staff.iter().enumerate() {
        if s.id == *user_id {
            tournament.info.staff.remove(i);
            done = true;
        }
    }

    if !done {
        return Err(ApiError::UserNotFound);
    }

    Ok(HttpResponse::Ok().finish())
}
