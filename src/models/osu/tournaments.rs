use async_trait::async_trait;
use bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::models::osu::BeatmapMod;
use crate::models::tournaments::{MappoolInfo, MatchInfo, TournamentInfo, TournamentTeamInfo};
use crate::repository::model::ModelExt;
use crate::repository::{RepoError, RepoResult, to_object_id};

pub enum TeamFormat {}

/// An osu_old!team is represented here
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct OsuTeam {
    pub info: TournamentTeamInfo,
}

/// An osu_old!map is represented here
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OsuMap {
    /// The osu_old!map's id
    pub osu_beatmap_id: i64,
    /// The modifier of the map
    pub modifier: Vec<BeatmapMod>,
}

/// An osu_old!mappool is represented here
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OsuMappool {
    pub info: MappoolInfo,

    /// The osu_old!mappool's name
    pub name: String,
    /// The osu_old!mappool's maps
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
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct OsuMatch {
    pub info: MatchInfo,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappool: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_team: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_team: Option<String>,

    pub osu_match_id: i64,
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
    pub current_stage: Option<String>,
}

impl OsuTournament {
    pub async fn get_team_by_raw_id(&self, id: &str) -> Option<(usize, OsuTeam)> {
        self.get_team(to_object_id(id)).await
    }

    pub async fn get_team(&self, id: ObjectId) -> Option<(usize, OsuTeam)> {
        for (i, team) in self.teams.iter().enumerate() {
            if team.info.id == Some(id) {
                return Some((i, team.clone()));
            }
        }

        None
    }
}

#[async_trait]
impl ModelExt for OsuTournament {
    type T = OsuTournament;

    async fn find_by_id(col: Collection<Self::T>, id: &ObjectId) -> RepoResult<Option<Self::T>> {
        Self::find_one(col, doc! {
                "$or": [
                    { "_id": id },
                    { "slug": id.to_hex() },
                ]
            }, None
        ).await
    }
}
