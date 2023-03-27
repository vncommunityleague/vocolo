use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// Players & Teams
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuPlayer {
    pub user_id: uuid::Uuid,
    pub osu_id: i64,
    // pub performance_point: f64,
    // pub global_rank: i64,
    // pub country_rank: i64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuTeam {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub slug: String,
    pub name: String,
    pub avatar_url: String,
    pub captain: OsuPlayer,
    pub players: Vec<OsuPlayer>,
}

// Mappool

/// An osua!map is represented here
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMap {
    /// The osua!map's id
    pub osu_beatmap_id: i64,
    /// The osua!map's set id
    pub osu_beatmapsets_id: i64,
}

/// An osua!mappool is represented here
/// We separate the mappool from the tournament stage
/// in order to allow for multiple stages to use the same mappool
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMappool {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// The osua!mappool's slug
    pub slug: String,
    /// The osua!mappool's name
    pub name: String,
    /// The osua!mappool's maps
    pub maps: Vec<OsuMap>,
}

// Tournament
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuTournamentStage {
    pub slug: String,
    pub name: String,
    pub mappool: OsuMappool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMatch {
    pub blue_team: OsuTeam,
    pub red_team: OsuTeam,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuTournament {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// Human readable id
    pub slug: String,
    /// Tournament name
    pub name: String,
    pub stages: Vec<OsuTournamentStage>,
    pub teams: Vec<OsuTeam>,
    pub players: Vec<OsuPlayer>,
}
