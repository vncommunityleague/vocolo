use bson::{doc, oid::ObjectId, to_document, Document};

use vocolo_internal::*;

use crate::{str_to_oid, to_vocolo_error, Database};

use super::{Matchup, PartialMatchup};

static COL: &str = "osu_matches";

pub trait AbstractOsuMatchup: Sync + Send {
    async fn fetch_osu_matchup(&self, match_id: &str) -> Result<Matchup>;

    async fn insert_osu_matchup(&self, matchup: &Matchup) -> Result<ObjectId>;

    async fn update_osu_matchup(&self, match_id: &str, tournament: &PartialMatchup) -> Result<()>;

    async fn delete_osu_matchup(&self, match_id: &str) -> Result<()>;
}

impl AbstractOsuMatchup for Database {
    async fn fetch_osu_matchup(&self, match_id: &str) -> Result<Matchup> {
        let match_oid = str_to_oid(match_id);

        let mappool = self
            .find_one(
                COL,
                doc! {
                    "_id": match_oid
                },
            )
            .await?
            .ok_or(Error::UnknownMatch)?;

        Ok(mappool)
    }

    async fn insert_osu_matchup(&self, matchup: &Matchup) -> Result<ObjectId> {
        let id = self
            .insert_one(COL, matchup)
            .await?
            .inserted_id
            .as_object_id()
            .unwrap();

        Ok(id)
    }

    async fn update_osu_matchup(&self, match_id: &str, partial: &PartialMatchup) -> Result<()> {
        let id = str_to_oid(match_id);

        let result = self
            .col::<Document>(COL)
            .update_one(
                doc! {
                    "_id": id
                },
                doc! {
                    "$set": to_document(partial)?
                },
                None,
            )
            .await
            .map_err(to_vocolo_error)?;

        if result.matched_count == 0 {
            return Err(Error::UnknownTournament);
        }

        Ok(())
    }

    async fn delete_osu_matchup(&self, match_id: &str) -> Result<()> {
        let oid = str_to_oid(match_id);

        let result = self
            .col::<Matchup>(COL)
            .delete_one(
                doc! {
                    "_id": oid
                },
                None,
            )
            .await
            .map_err(to_vocolo_error)?;

        if result.deleted_count == 0 {
            return Err(Error::UnknownTournament);
        }

        Ok(())
    }
}
