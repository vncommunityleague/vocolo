use bson::{doc, oid::ObjectId, to_document, Bson, Document};
use futures::{StreamExt, TryStreamExt};
use mongodb::options::FindOneOptions;

use vocolo_internal::*;

use crate::{str_to_oid, to_vocolo_error, Database};

use super::{PartialTournament, Tournament, TournamentTeam};

static COL: &str = "osu_tournaments";

pub trait AbstractOsuTournament: Sync + Send {
    async fn fetch_osu_tournament(&self, id: &str) -> Result<Option<Tournament>>;

    async fn insert_osu_tournament(&self, tournament: &Tournament) -> Result<ObjectId>;

    async fn update_osu_tournament(
        &self,
        id: &str,
        tournament: &PartialTournament,
    ) -> Result<Option<()>>;

    async fn delete_osu_tournament(&self, id: &str) -> Result<Option<()>>;

    async fn check_osu_tournament_exists(&self, filter: Document) -> Result<bool>;

    async fn check_osu_tournament_players_exist(&self, id: &str, ids: &[i32]) -> Result<Vec<i32>>;

    async fn fetch_osu_tournament_teams(&self, id: &str) -> Result<Option<Vec<TournamentTeam>>>;

    async fn insert_osu_tournament_teams(
        &self,
        id: &str,
        team: Vec<&TournamentTeam>,
    ) -> Result<Option<u64>>;

    async fn delete_osu_tournament_teams(
        &self,
        id: &str,
        team_ids: Vec<&str>,
    ) -> Result<Option<u64>>;
}

impl AbstractOsuTournament for Database {
    async fn fetch_osu_tournament(&self, id: &str) -> Result<Option<Tournament>> {
        let oid = str_to_oid(id);

        let opts = FindOneOptions::builder()
            .projection(doc! {
                "teams": 0,
            })
            .build();

        let tournament = self
            .find_one_with_options(
                COL,
                doc! {
                    "$or": [
                        { "_id": oid },
                        { "slug": id }
                    ]
                },
                opts,
            )
            .await?;

        Ok(tournament)
    }

    async fn insert_osu_tournament(&self, tournament: &Tournament) -> Result<ObjectId> {
        let id = self
            .insert_one(COL, tournament)
            .await?
            .inserted_id
            .as_object_id()
            .unwrap();

        Ok(id)
    }

    async fn update_osu_tournament(
        &self,
        id: &str,
        data: &PartialTournament,
    ) -> Result<Option<()>> {
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
            return Ok(None);
        }

        Ok(Some(()))
    }

    async fn delete_osu_tournament(&self, id: &str) -> Result<Option<()>> {
        let oid = str_to_oid(id);

        let result = self
            .col::<Tournament>(COL)
            .delete_one(
                doc! {
                    "_id": oid
                },
                None,
            )
            .await
            .map_err(to_vocolo_error)?;

        if result.deleted_count == 0 {
            return Ok(None);
        }

        Ok(Some(()))
    }

    async fn check_osu_tournament_exists(&self, filter: Document) -> Result<bool> {
        self.exists(COL, filter).await
    }

    async fn check_osu_tournament_players_exist(&self, id: &str, ids: &[i32]) -> Result<Vec<i32>> {
        let id = str_to_oid(id);

        // TODO: uhhh
        let mut result = self.col::<Document>(COL).aggregate(vec![
            doc! { "$match": { "_id": id } },
            doc! { "$unwind": "$teams" },
            doc! { "$unwind": "$teams.players" },
            doc! { "$match": { "teams.players": { "$in": ids } } },
            doc! { "$group": { "_id": null, "duplicatePlayers": { "$addToSet": "$teams.players" } } },
            doc! { "$project": { "_id": 0, "duplicatePlayers": 1 } }
        ], None).await.map_err(to_vocolo_error)?;

        let duplicate_players = if let Some(result) = result.next().await {
            match result {
                Ok(doc) => doc
                    .get_array("duplicatePlayers")
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|bson| match bson {
                        Bson::Int32(player_id) => Some(*player_id),
                        _ => None,
                    })
                    .collect(),
                Err(_) => vec![],
            }
        } else {
            vec![]
        };

        Ok(duplicate_players)
    }

    async fn fetch_osu_tournament_teams(&self, id: &str) -> Result<Option<Vec<TournamentTeam>>> {
        let oid = str_to_oid(id);

        let opts = FindOneOptions::builder()
            .projection(doc! {
                "teams": 1,
            })
            .build();

        let tournament = self
            .col::<PartialTournament>(COL)
            .find_one(
                doc! {
                    "$or": [
                        { "_id": oid },
                        { "slug": id }
                    ]
                },
                opts,
            )
            .await
            .map_err(to_vocolo_error)?;

        Ok(tournament.map(|t| t.teams.unwrap_or_default()))
    }

    async fn insert_osu_tournament_teams(
        &self,
        id: &str,
        teams: Vec<&TournamentTeam>,
    ) -> Result<Option<u64>> {
        let id = str_to_oid(id);
        let teams = teams
            .iter()
            .map(|v| to_document(v).unwrap())
            .collect::<Vec<_>>();

        let result = self
            .col::<Tournament>(COL)
            .update_one(
                doc! {
                    "_id": id
                },
                doc! {
                    "$push": {
                        "teams": {
                            "$each": teams
                        }
                    }
                },
                None,
            )
            .await
            .map_err(to_vocolo_error)?;

        if result.matched_count == 0 {
            return Ok(None);
        }

        Ok(Some(result.modified_count))
    }

    async fn delete_osu_tournament_teams(
        &self,
        id: &str,
        team_ids: Vec<&str>,
    ) -> Result<Option<u64>> {
        let id = str_to_oid(id);
        let team_ids = team_ids
            .iter()
            .map(|id| str_to_oid(id))
            .collect::<Vec<ObjectId>>();

        let result = self
            .col::<Tournament>(COL)
            .update_one(
                doc! {
                    "_id": id
                },
                doc! {
                    "$pull": {
                        "teams": {
                            "_id": {
                                "$in": team_ids
                            }
                        }
                    }
                },
                None,
            )
            .await
            .map_err(to_vocolo_error)?;

        if result.matched_count == 0 {
            return Ok(None);
        }

        Ok(Some(result.modified_count))
    }
}
