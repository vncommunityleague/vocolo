use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::osu::BeatmapMod;
use crate::models::tournaments::{MatchInfo, TournamentInfo, TournamentTeamInfo};

pub enum TeamFormat {}

/// An osu!team is represented here
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OsuTeam {
    pub info: TournamentTeamInfo,
}

impl Default for OsuTeam {
    fn default() -> Self {
        Self {
            info: TournamentTeamInfo::default(),
        }
    }
}

/// An osu!map is represented here
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OsuMap {
    /// The osu!map's id
    pub osu_beatmap_id: i64,
    /// The modifier of the map
    pub modifier: Vec<BeatmapMod>,
}

/// An osu!mappool is represented here
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OsuMappool {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// The osu!mappool's name
    pub name: String,
    /// The osu!mappool's maps
    pub maps: Vec<OsuMap>,
}

impl OsuMappool {
    pub async fn get_map(&self, osu_beatmap_id: i64) -> Option<(usize, OsuMap)> {
        for (i, map) in self.maps.iter().enumerate() {
            if map.osu_beatmap_id == osu_beatmap_id {
                return Some((i, map.clone()));
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMatchMap {}

// Tournament
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OsuMatch {
    pub info: MatchInfo,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappool: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_team: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_team: Option<ObjectId>,

    pub osu_match_id: i64,
}

impl Default for OsuMatch {
    fn default() -> Self {
        Self {
            info: MatchInfo::default(),
            mappool: None,
            blue_team: None,
            red_team: None,
            osu_match_id: -1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OsuTournamentStage {
    pub slug: String,
    pub name: String,

    pub matches: Vec<OsuMatch>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OsuTournament {
    #[serde(flatten)]
    pub info: TournamentInfo,

    // pub game_mode: GameMode,
    #[serde(default)]
    pub teams: Vec<OsuTeam>,

    #[serde(default)]
    pub players: Vec<String>,

    /// The current stage of the tournament
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<ObjectId>,
}

impl OsuTournament {
    pub async fn get_team(&self, name: String) -> Option<(usize, OsuTeam)> {
        for (i, team) in self.teams.iter().enumerate() {
            if team.info.name == name {
                return Some((i, team.clone()));
            }
        }

        None
    }
}
