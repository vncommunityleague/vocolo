use crate::models::osu::BeatmapMod;
use crate::models::tournaments::{MatchInfo, TournamentInfo, TournamentStaff, TournamentTeamInfo};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub enum TeamFormat {}

/// An osu!team is represented here
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuTeam {
    pub info: TournamentTeamInfo,
}

/// An osu!map is represented here
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMap {
    /// The osu!map's id
    pub osu_beatmap_id: i64,
    /// The modifier of the map
    pub modifier: BeatmapMod,
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

impl OsuMappool {
    pub async fn get_map_position(&self, osu_beatmap_id: i64) -> Option<usize> {
        for (i, map) in self.maps.iter().enumerate() {
            if map.osu_beatmap_id == osu_beatmap_id {
                return Some(i);
            }
        }

        None
    }

    pub async fn get_map(&self, osu_beatmap_id: i64) -> Option<OsuMap> {
        for map in &self.maps {
            if map.osu_beatmap_id == osu_beatmap_id {
                return Some(map.clone());
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMatchMap {

}

// Tournament
#[derive(Serialize, Deserialize, Clone)]
pub struct OsuMatch {
    pub info: MatchInfo,

    pub blue_team: OsuTeam,
    pub red_team: OsuTeam,

    pub osu_match_id: i64,
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
    #[serde(flatten)]
    pub info: TournamentInfo,

    // pub game_mode: GameMode,
    #[serde(default)]
    pub teams: Vec<OsuTeam>,

    #[serde(default)]
    pub mappools: Vec<OsuMappool>,

    /// The current stage of the tournament
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<ObjectId>,
}

impl OsuTournament {
    pub async fn get_team_position(&self, name: String) -> Option<usize> {
        for (i, team) in self.teams.iter().enumerate() {
            if team.info.name == name {
                return Some(i);
            }
        }

        None
    }

    pub async fn get_team(&self, name: String) -> Option<OsuTeam> {
        for team in &self.teams {
            if team.info.name == name {
                return Some(team.clone());
            }
        }

        None
    }

    pub async fn get_mappool_position(&self, slug: String) -> Option<usize> {
        for (i, mappool) in self.mappools.iter().enumerate() {
            if mappool.slug == slug {
                return Some(i);
            }
        }

        None
    }

    pub async fn get_mappool(&self, slug: String) -> Option<OsuMappool> {
        for mappool in &self.mappools {
            if mappool.slug == slug {
                return Some(mappool.clone());
            }
        }

        None
    }

    pub async fn players(&self) -> Vec<String> {
        let mut players = Vec::new();

        for team in &self.teams {
            players.append(&mut team.info.players.clone());
        }

        players
    }
}
