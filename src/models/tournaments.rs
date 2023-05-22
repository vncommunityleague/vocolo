use crate::models::ModelAttribute;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::user::Role;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TournamentStaff {
    pub id: String,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct TournamentTeamInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,

    pub captain: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    pub players: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MappoolInfo {
    #[serde(flatten)]
    pub model_attribute: ModelAttribute,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MatchInfo {
    #[serde(flatten)]
    pub model_attribute: ModelAttribute,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tournament: Option<ObjectId>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct TournamentInfo {
    #[serde(flatten)]
    pub model_attribute: ModelAttribute,

    pub slug: String,
    pub title: String,

    // pub registration_start: i64,
    // pub registration_end: i64,
    pub staff: Vec<TournamentStaff>,
}
