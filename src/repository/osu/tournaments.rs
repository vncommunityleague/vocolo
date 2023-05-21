use mongodb::bson::{doc, Document};
use mongodb::{Collection, Database};
use tokio_stream::StreamExt;

use crate::models::osu::tournaments::{OsuMappool, OsuMatch, OsuTournament};
use crate::repository::{to_object_id, RepoError, RepoResult};

#[derive(Clone)]
pub struct OsuTournamentRepo {
    pub tournaments: Collection<OsuTournament>,
    pub mappools: Collection<OsuMappool>,
    pub matches: Collection<OsuMatch>,
}

impl OsuTournamentRepo {
    pub(crate) async fn init(database: &Database) -> Self {
        let tournaments: Collection<OsuTournament> = database.collection("tournaments");
        let mappools: Collection<OsuMappool> = database.collection("mappools");
        let matches: Collection<OsuMatch> = database.collection("matches");

        OsuTournamentRepo {
            tournaments,
            mappools,
            matches,
        }
    }

    // Tournaments

    /// Lists all [`OsuTournament`] in the database.
    pub async fn list_tournaments(&self) -> RepoResult<Vec<OsuTournament>> {
        self.find_tournaments(doc! {}).await
    }

    /// Finds the [`OsuTournament`] that matches the id or slug.
    pub async fn find_tournament_by_id_or_slug(
        &self,
        id_or_slug: &str,
    ) -> RepoResult<OsuTournament> {
        self.find_tournament(doc! {
            "$or": [
                { "_id": to_object_id(id_or_slug) },
                { "slug": id_or_slug }
            ]
        })
        .await
    }

    /// Finds the [`OsuTournament`] that matches the filter.
    pub async fn find_tournament(&self, filter: Document) -> RepoResult<OsuTournament> {
        let tournament = self.tournaments.find_one(Some(filter), None).await;

        return match tournament {
            Ok(tournament) => Ok(tournament),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }

    /// Finds and returns all [`OsuTournament`] that match the filter.
    pub async fn find_tournaments(&self, filter: Document) -> RepoResult<Vec<OsuTournament>> {
        let cursor = self.tournaments.find(Some(filter), None).await;

        if cursor.is_err() {
            return Err(RepoError::Internal(cursor.unwrap_err()));
        }

        let mut cursor = cursor.unwrap();
        let mut tournaments = Vec::new();

        while let Some(result) = cursor.next().await {
            tournaments.push(
                result.unwrap_or_else(|e| {
                    panic!("Unexpected error while finding tournament: {}.", e)
                }),
            );
        }

        Ok(Some(tournaments))
    }

    /// Creates a new [`OsuTournament`] and returns its id.
    pub async fn create_tournament(
        &self,
        tournament: OsuTournament,
    ) -> RepoResult<OsuTournament> {
        let slug = &tournament.info.slug.clone();
        let check_tournament = self.find_tournament_by_id_or_slug(slug).await;

        if check_tournament.is_ok() && check_tournament.unwrap().is_some() {
            return Err(RepoError::Duplicate(slug.to_string()));
        }

        let query_result = self
            .tournaments
            .clone_with_type()
            .insert_one(tournament, None)
            .await;

        return match query_result {
            Ok(_) => Ok(self.find_tournament_by_id_or_slug(&slug).await.unwrap()),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }

    pub async fn replace_tournament(
        &self,
        id_or_slug: &str,
        new_data: OsuTournament,
    ) -> RepoResult<OsuTournament> {
        let query_result = self
            .tournaments
            .replace_one(
                doc! {
                    "$or": [
                        { "_id": to_object_id(id_or_slug) },
                        { "slug": id_or_slug }
                    ]
                },
                new_data,
                None,
            )
            .await
            .unwrap_or_else(|e| panic!("Unexpected error while replacing tournament: {}.", e));

        if query_result.modified_count == 0 {
            return Ok(None);
        }

        self.find_tournament_by_id_or_slug(id_or_slug).await
    }

    pub async fn delete_tournament_by_id_or_slug(
        &self,
        id_or_slug: &str,
    ) -> RepoResult<OsuTournament> {
        self.delete_tournament(doc! {
            "$or": [
                { "_id": to_object_id(id_or_slug) },
                { "slug": id_or_slug }
            ]
        })
        .await
    }

    /// Deletes the [`OsuTournament`] that matches the id or slug.
    pub async fn delete_tournament(&self, filter: Document) -> RepoResult<OsuTournament> {
        let query_result = self.tournaments.find_one_and_delete(filter, None).await;

        return match query_result {
            Ok(tournament) => Ok(tournament),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }

    // Mappools

    pub async fn list_mappools(&self) -> RepoResult<Vec<OsuMappool>> {
        self.find_mappools(doc! {}).await
    }

    pub async fn find_mappool_by_id(&self, id: &str) -> RepoResult<OsuMappool> {
        self.find_mappool(doc! { "_id": to_object_id(id) }).await
    }

    pub async fn find_mappool(&self, filter: Document) -> RepoResult<OsuMappool> {
        let query_result = self.mappools.find_one(Some(filter), None).await;

        return match query_result {
            Ok(value) => Ok(value),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }

    pub async fn find_mappools(&self, filter: Document) -> RepoResult<Vec<OsuMappool>> {
        let cursor = self.mappools.find(Some(filter), None).await;

        if cursor.is_err() {
            return Err(RepoError::Internal(cursor.unwrap_err()));
        }

        let mut cursor = cursor.unwrap();
        let mut mappools = Vec::new();

        while let Some(result) = cursor.next().await {
            mappools.push(
                result.unwrap_or_else(|e| {
                    panic!("Unexpected error while finding tournament: {}.", e)
                }),
            );
        }

        Ok(Some(mappools))
    }

    pub async fn create_mappool(&self, mappool: OsuMappool) -> RepoResult<OsuMappool> {
        // TODO: check for existed one
        let query_result = self
            .mappools
            .clone_with_type()
            .insert_one(mappool, None)
            .await;

        return match query_result {
            Ok(_) => Ok(self.find_mappool(doc! {}).await.unwrap()),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }

    pub async fn replace_mappool(&self, id: &str, new_data: OsuMappool) -> RepoResult<OsuMappool> {
        let query_result = self
            .mappools
            .replace_one(
                doc! {
                    "_id": to_object_id(id)
                },
                new_data,
                None,
            )
            .await
            .unwrap_or_else(|e| panic!("Unexpected error while replacing mappool: {}.", e));

        if query_result.modified_count == 0 {
            return Ok(None);
        }

        self.find_mappool(doc! {"_id": to_object_id(id)}).await
    }

    pub async fn delete_mappool_by_id(&self, id: &str) -> RepoResult<OsuMappool> {
        self.delete_mappool(doc! { "_id": to_object_id(id) })
            .await
    }

    pub async fn delete_mappool(&self, filter: Document) -> RepoResult<OsuMappool> {
        let query_result = self
            .mappools
            .find_one_and_delete(filter, None)
            .await;

        return match query_result {
            Ok(mappool) => Ok(mappool),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }

    // Matches

    pub async fn list_matches(&self) -> RepoResult<Vec<OsuMatch>> {
        self.find_matches(doc! {}).await
    }

    pub async fn find_match_by_id(&self, id: &str) -> RepoResult<OsuMatch> {
        self.find_match(doc! { "_id": to_object_id(id) }).await
    }

    pub async fn find_match(&self, filter: Document) -> RepoResult<OsuMatch> {
        let query_result = self.matches.find_one(Some(filter), None).await;

        return match query_result {
            Ok(value) => Ok(value),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }

    pub async fn find_matches(&self, filter: Document) -> RepoResult<Vec<OsuMatch>> {
        let cursor = self.matches.find(Some(filter), None).await;

        if cursor.is_err() {
            return Err(RepoError::Internal(cursor.unwrap_err()));
        }

        let mut cursor = cursor.unwrap();
        let mut mappools = Vec::new();

        while let Some(result) = cursor.next().await {
            mappools.push(
                result.unwrap_or_else(|e| {
                    panic!("Unexpected error while finding tournament: {}.", e)
                }),
            );
        }

        Ok(Some(mappools))
    }

    pub async fn create_match(&self, game_match: OsuMatch) -> RepoResult<OsuMatch> {
        // TODO: actually fix this
        // let id = &game_match.info.id;
        // let check_game_match = self.find_match_by_id(slug).await;
        // 
        // if check_game_match.is_ok() && check_tournament.unwrap().is_some() {
        //     return Err(RepoError::Duplicate(slug.to_string()));
        // }

        let query_result = self
            .matches
            .clone_with_type()
            .insert_one(game_match, None)
            .await;

        return match query_result {
            Ok(_) => Ok(self.find_match_by_id("").await.unwrap()),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }

    pub async fn delete_match_by_id(&self, id: &str) -> RepoResult<OsuMatch> {
        self.delete_match(doc! { "_id": to_object_id(id) })
            .await
    }

    pub async fn delete_match(&self, filter: Document) -> RepoResult<OsuMatch> {
        let query_result = self
            .matches
            .find_one_and_delete(filter, None)
            .await;

        return match query_result {
            Ok(mappool) => Ok(mappool),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }
}
