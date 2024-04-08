use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use revolt_optional_struct::OptionalStruct;
use serde::{Deserialize, Serialize};

use vocolo_internal::*;

use crate::Database;

use super::AbstractOsuMatchup;

#[derive(OptionalStruct, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[optional_derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
#[optional_name = "PartialMatchup"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct Matchup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub tournament_id: ObjectId,

    pub date: DateTime<Utc>,

    pub team_red: Option<ObjectId>,
    pub team_blue: Option<ObjectId>,

    pub maps: Vec<MatchupMap>,
}

#[derive(OptionalStruct, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[optional_derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
#[optional_name = "PartialMatchupMap"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct MatchupMap {
    pub map_id: i32,
    pub map_type: MatchupMapType,

    pub team: ObjectId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_red_scores: Option<Vec<MatchupMapScore>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_blue_scores: Option<Vec<MatchupMapScore>>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MatchupMapType {
    Pick,
    Ban,
    Protect,
}

#[derive(OptionalStruct, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[optional_derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
#[optional_name = "PartialMatchupMapScore"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct MatchupMapScore {
    pub player: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mods: Option<String>,

    pub score: u32,
}

impl Matchup {
    pub async fn create(&self, db: &Database) -> Result<ObjectId> {
        let id = db.insert_osu_matchup(self).await?;
        Ok(id)
    }

    pub async fn fetch(db: &Database, match_id: &str) -> Result<Matchup> {
        let matchup = db.fetch_osu_matchup(match_id).await?;
        Ok(matchup)
    }

    pub async fn update(db: &Database, match_id: &str, partial: &PartialMatchup) -> Result<()> {
        db.update_osu_matchup(match_id, partial).await?;
        Ok(())
    }

    pub async fn delete(db: &Database, match_id: &str) -> Result<()> {
        db.delete_osu_matchup(match_id).await?;
        Ok(())
    }
}
