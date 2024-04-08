use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Matchup {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub tournament_id: ObjectId,

    pub date: DateTime<Utc>,

    pub team_red: Option<ObjectId>,
    pub team_blue: Option<ObjectId>,

    pub maps: Vec<MatchupMap>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct MatchupMap {
    pub map_id: i32,
    pub map_type: MatchupMapType,

    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
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

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct MatchupMapScore {
    pub player: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mods: Option<String>,

    pub score: u32,
}

#[derive(garde::Validate, Deserialize, Debug)]
#[garde(allow_unvalidated)]
pub struct CreateMatchupRequest {
    pub tournament_id: ObjectId,
    pub date: DateTime<Utc>,
    pub team_red: Option<ObjectId>,
    pub team_blue: Option<ObjectId>,
}

#[derive(Serialize)]
pub struct CreateMatchupResponse {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
}

#[derive(garde::Validate, Deserialize, Debug)]
#[garde(allow_unvalidated)]
pub struct UpdateMatchupRequest {
    pub date: Option<DateTime<Utc>>,
}
