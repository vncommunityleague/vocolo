use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::v1::osu::GameMode;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Tournament {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub slug: String,
    pub name: String,

    pub mode: GameMode,

    #[serde(skip_serializing_if = "crate::if_false", default)]
    pub invite_only: bool,
    pub min_team_size: i16,
    pub max_team_size: i16,

    pub registration_start_date: DateTime<Utc>,
    pub registration_end_date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct TournamentTeam {
    pub id: ObjectId,
    pub name: String,

    pub captain: i32,
    pub players: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct TournamentStaff {
    pub id: i32,
    pub role: TournamentStaffRole,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum TournamentStaffRole {
    Organizer,

    Mappooler,
    Mapper,
    Testplayer,

    Referee,
    Streamer,
    Commentator,

    Staff,
    Designer,
    Developer,
}

// DTO

#[derive(garde::Validate, Deserialize, Debug)]
#[garde(allow_unvalidated)]
pub struct CreateTournamentRequest {
    #[garde(length(min = crate::MIN_SLUG_LENGTH, max = crate::MAX_SLUG_LENGTH))]
    pub slug: String,
    #[garde(length(min = crate::MIN_NAME_LENGTH, max = crate::MAX_NAME_LENGTH))]
    pub name: String,
    pub mode: GameMode,
    pub invite_only: Option<bool>,
    #[garde(range(min = 1, max = 16))]
    pub min_team_size: i16,
    #[garde(range(min = 1, max = 128))]
    pub max_team_size: i16,
    pub registration_start_date: Option<DateTime<Utc>>,
    pub registration_end_date: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct CreateTournamentResponse {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub slug: String,
    pub name: String,
}

#[derive(garde::Validate, Deserialize, Debug)]
#[garde(allow_unvalidated)]
pub struct UpdateTournamentRequest {
    #[garde(length(min = crate::MIN_SLUG_LENGTH, max = crate::MAX_SLUG_LENGTH))]
    pub slug: Option<String>,
    pub name: Option<String>,
    pub mode: Option<GameMode>,
    pub invite_only: Option<bool>,
    #[garde(range(min = 1, max = 128))]
    pub min_team_size: Option<i16>,
    #[garde(range(min = 1, max = 128))]
    pub max_team_size: Option<i16>,
    pub registration_start_date: Option<DateTime<Utc>>,
    pub registration_end_date: Option<DateTime<Utc>>,
}

#[derive(garde::Validate, Deserialize, Debug)]
#[garde(allow_unvalidated)]
pub struct RegisterTournamentTeamRequest {
    #[garde(length(min = crate::MIN_NAME_LENGTH, max = crate::MAX_NAME_LENGTH))]
    pub name: String,
    pub players: Option<Vec<i32>>,
}

#[derive(Serialize)]
pub struct RegisterTournamentTeamResponse {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
}

#[derive(Serialize)]
pub struct ListPlayerResponse {
    pub players: Vec<i32>,
}
