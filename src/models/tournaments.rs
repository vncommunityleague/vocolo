use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::user::Role;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TournamentStaff {
    pub id: String,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TournamentTeamInfo {
    pub name: String,

    pub captain: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(default)]
    pub players: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MatchInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TournamentInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub slug: String,
    pub title: String,
    // pub created_at: i64,
    // pub updated_at: i64,

    // pub registration_start: i64,
    // pub registration_end: i64,
    #[serde(default)]
    pub staff: Vec<TournamentStaff>,
}
