use bson::{doc, oid::ObjectId, to_document, Document};

use vocolo_internal::{Error, Result};

use crate::{str_to_oid, to_vocolo_error, Database};

use super::{Mappool, MappoolMap, PartialMappool};

static COL: &str = "osu_mappools";

pub trait AbstractOsuMappool: Sync + Send {
    async fn fetch_osu_mappool(&self, mappool_id: &str) -> Result<Mappool>;

    async fn insert_osu_mappool(&self, mappool: &Mappool) -> Result<ObjectId>;

    async fn update_osu_mappool(&self, id: &str, data: &PartialMappool) -> Result<()>;

    async fn delete_osu_mappool(&self, mappool_id: &str) -> Result<u64>;

    async fn insert_osu_mappool_maps(&self, mappool_id: &str, maps: &Vec<MappoolMap>)
        -> Result<()>;

    async fn delete_osu_mappool_map(&self, mappool_id: &str, map_ids: &i32) -> Result<()>;
}

impl AbstractOsuMappool for Database {
    async fn fetch_osu_mappool(&self, mappool_id: &str) -> Result<Mappool> {
        let mappool_oid = str_to_oid(mappool_id);

        let mappool = self
            .find_one(
                COL,
                doc! {
                    "_id": mappool_oid
                },
            )
            .await?
            .ok_or(Error::UnknownMappool)?;

        Ok(mappool)
    }

    async fn insert_osu_mappool(&self, mappool: &Mappool) -> Result<ObjectId> {
        let id = self
            .insert_one(COL, mappool)
            .await?
            .inserted_id
            .as_object_id()
            .unwrap();

        Ok(id)
    }

    async fn update_osu_mappool(&self, id: &str, data: &PartialMappool) -> Result<()> {
        let id = str_to_oid(id);

        let result = self
            .col::<Document>(COL)
            .update_one(
                doc! {
                    "_id": id
                },
                doc! {
                    "$set": to_document(data)?
                },
                None,
            )
            .await
            .map_err(to_vocolo_error)?;

        if result.matched_count == 0 {
            return Err(Error::UnknownMappool);
        }

        Ok(())
    }

    async fn delete_osu_mappool(&self, mappool_id: &str) -> Result<u64> {
        let mappool_oid = str_to_oid(mappool_id);

        let result = self.delete_one_by_id(COL, &mappool_oid).await?;

        Ok(result.deleted_count)
    }

    async fn insert_osu_mappool_maps(&self, mappool_id: &str, map: &Vec<MappoolMap>) -> Result<()> {
        let mappool_oid = str_to_oid(mappool_id);

        self.col::<Document>(COL)
            .update_one(
                doc! {
                    "_id": mappool_oid
                },
                doc! {
                    "$push": {
                        "maps": to_document(&map)?
                    }
                },
                None,
            )
            .await
            .map_err(to_vocolo_error)?;

        Ok(())
    }

    async fn delete_osu_mappool_map(&self, mappool_id: &str, pos: &i32) -> Result<()> {
        let mappool_oid = str_to_oid(mappool_id);

        let find_query = doc! {
            "_id": mappool_oid
        };

        let result = self
            .col::<Document>(COL)
            .update_one(
                find_query.clone(),
                doc! {
                    "$unset": {
                        format!("maps.{}", pos): 1
                    }
                },
                None,
            )
            .await
            .map_err(to_vocolo_error)?;

        if result.matched_count == 0 {
            return Err(Error::UnknownMappool);
        } else if result.modified_count == 0 {
            return Err(Error::UnknownMappoolMap);
        }

        self.col::<Document>(COL)
            .update_one(
                find_query.clone(),
                doc! {
                    "$pull": {
                        "maps": null
                    }
                },
                None,
            )
            .await
            .map_err(to_vocolo_error)?;

        Ok(())
    }
}
