use serde::{Deserialize, Serialize};
use crate::models::{ModelAttribute};

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

impl Default for TournamentTeamInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            captain: String::new(),
            avatar_url: None,
            players: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MatchInfo {
    #[serde(flatten)]
    pub model_attribute: ModelAttribute,

    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TournamentInfo {
    #[serde(flatten)]
    pub model_attribute: ModelAttribute,

    pub slug: String,
    pub title: String,

    // pub registration_start: i64,
    // pub registration_end: i64,
    #[serde(default)]
    pub staff: Vec<TournamentStaff>,
}
