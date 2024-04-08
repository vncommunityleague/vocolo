use bson::oid::ObjectId;
use revolt_optional_struct::OptionalStruct;
use serde::{Deserialize, Serialize};

use vocolo_internal::*;

use crate::Database;

use super::AbstractOsuMappool;

#[derive(OptionalStruct, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[optional_derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
#[optional_name = "PartialMappool"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct Mappool {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub tournament_id: ObjectId,

    pub private: bool,

    pub mappack_link: String,

    pub maps: Vec<MappoolMap>,
}

#[derive(OptionalStruct, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[optional_derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
#[optional_name = "PartialMappoolMap"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct MappoolMap {
    pub beatmap_id: i32,
    pub modifiers: String,
}

impl Mappool {
    pub async fn create(&self, db: &Database) -> Result<ObjectId> {
        let id = db.insert_osu_mappool(self).await?;
        Ok(id)
    }

    pub async fn update(db: &Database, mappool_id: &str, partial: &PartialMappool) -> Result<()> {
        db.update_osu_mappool(mappool_id, partial).await?;
        Ok(())
    }

    pub async fn fetch(db: &Database, mappool_id: &str) -> Result<Mappool> {
        let mappool = db.fetch_osu_mappool(mappool_id).await?;
        Ok(mappool)
    }

    pub async fn delete(db: &Database, mappool_id: &str) -> Result<()> {
        db.delete_osu_mappool(mappool_id).await?;
        Ok(())
    }

    pub async fn add_maps(db: &Database, mappool_id: &str, maps: Vec<MappoolMap>) -> Result<()> {
        db.insert_osu_mappool_maps(mappool_id, &maps).await?;
        Ok(())
    }

    pub async fn delete_map(db: &Database, mappool_id: &str, pos: &i32) -> Result<()> {
        db.delete_osu_mappool_map(mappool_id, pos).await?;
        Ok(())
    }
}
