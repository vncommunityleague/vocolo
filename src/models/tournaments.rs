use crate::models::user::Role;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct StaffListByRole {
    pub role: Role,
    pub staff: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MatchInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub slug: String,
    pub title: String,
    // pub created_at: i64,
    // pub updated_at: i64,

    // pub registration_start: i64,
    // pub registration_end: i64,
    pub staff: Vec<StaffListByRole>,
}
