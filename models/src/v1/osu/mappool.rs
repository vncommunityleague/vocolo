use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Mappool {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,

    pub mappack_link: String,

    pub maps: Vec<MappoolMap>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct MappoolMap {
    pub beatmap_id: i32,
    pub modifiers: String,
}

// DTO

#[derive(garde::Validate, Deserialize, Debug)]
#[garde(allow_unvalidated)]
pub struct CreateMappoolRequest {
    pub tournament_id: String,
    pub private: Option<bool>,
}

#[derive(garde::Validate, Deserialize, Debug)]
#[garde(allow_unvalidated)]
pub struct UpdateMappoolRequest {
    pub map_link: Option<String>,
}

#[derive(garde::Validate, Deserialize, Debug)]
#[garde(allow_unvalidated)]
pub struct AddMappoolMapRequest {
    pub maps: Vec<MappoolMap>,
}
