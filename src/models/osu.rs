use crate::models::user::User;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum GameMode {
    Standard,
    Taiko,
    Catch,
    Mania,
}

pub enum BeatmapMod {
    NoMod,
    Hidden,
    HardRock,
    DoubleTime,
    FreeMod,
    Easy,
    HalfTime,
    Flashlight,
    Tiebreaker,
}

impl BeatmapMod {
    pub async fn id(&self) -> &str {
        match *self {
            BeatmapMod::NoMod => "NM",
            BeatmapMod::Hidden => "HD",
            BeatmapMod::HardRock => "HR",
            BeatmapMod::DoubleTime => "DT",
            BeatmapMod::FreeMod => "FM",
            BeatmapMod::Easy => "EZ",
            BeatmapMod::HalfTime => "HT",
            BeatmapMod::Flashlight => "FL",
            BeatmapMod::Tiebreaker => "TB",
        }
    }
}

/// An osu!team is represented here
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuTeam {
    pub id: ObjectId,
    pub name: String,
    pub avatar_url: String,
    pub captain: Option<User>,
    /// Contains the osu!user ids
    #[serde(default)]
    pub players: Vec<u64>,
}

// Mappool

/// An osu!map is represented here
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMap {
    /// The osu!map's id
    pub osu_beatmap_id: i64,
    /// The osu!map's set id
    pub osu_beatmapsets_id: i64,
}

/// An osu!mappool is represented here
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMappool {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// The osu!mappool's slug
    pub slug: String,
    /// The osu!mappool's name
    pub name: String,
    /// The osu!mappool's maps
    pub maps: Vec<OsuMap>,
}

// Tournament
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMatch {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub blue_team: OsuTeam,
    pub red_team: OsuTeam,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuTournamentStage {
    pub slug: String,
    pub name: String,

    pub mappool: Option<ObjectId>,

    pub matches: Vec<OsuMatch>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuTournament {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    // pub created_at: i64,
    //
    // pub updated_at: i64,

    /// Human readable id
    pub slug: String,
    /// Tournament title
    pub title: String,

    #[serde(default)]
    pub teams: Vec<OsuTeam>,

    #[serde(default)]
    pub mappools: Vec<OsuMappool>,

    /// The current stage of the tournament
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<ObjectId>,

    #[serde(default)]
    pub stages: Vec<OsuTournamentStage>,
}

impl OsuTournament {
    pub async fn get_team(&self, name: String) -> Option<OsuTeam> {
        for team in &self.teams {
            if team.name == name {
                return Some(team.clone());
            }
        }

        None
    }

    // pub async fn players(&self) -> Vec<User> {
    //     let mut players = Vec::new();
    //
    //     for team in &self.teams {
    //         players.append(&mut team.players.clone());
    //     }
    //
    //     players
    // }
}
