use crate::models::osu::tournaments::OsuTournament;
use crate::repository::{RepoError, RepoResult};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::{Collection, Database};
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct OsuTournamentRepo {
    pub tournaments: Collection<OsuTournament>,
}

impl OsuTournamentRepo {
    pub(crate) async fn init(database: &Database) -> Self {
        let tournaments: Collection<OsuTournament> = database.collection("tournaments");

        OsuTournamentRepo { tournaments }
    }

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
                { "_id": ObjectId::parse_str(id_or_slug).unwrap_or_default() },
                { "slug": id_or_slug }
            ]
        })
        .await
    }

    /// Finds and returns all [`OsuTournament`] that match the id or slug.
    pub async fn find_tournaments_by_ids_or_slugs(
        &self,
        id_or_slug_list: Vec<String>,
    ) -> RepoResult<Vec<OsuTournament>> {
        self.find_tournaments(doc! {
            "$or": [
                { "_id": { "$in": id_or_slug_list.iter().map(|id| ObjectId::parse_str(id).unwrap_or_default()).collect::<Vec<ObjectId>>() } },
                { "slug": { "$in": id_or_slug_list } }
            ]
        }).await
    }

    /// Finds the [`OsuTournament`] that matches the id or slug.
    pub async fn find_tournament(&self, filter: Document) -> RepoResult<OsuTournament> {
        let tournament = self
            .tournaments
            .find_one(Some(filter), None)
            .await;

        if tournament.is_err() {
            return Err(RepoError::QueryFatal {
                message: format!("finding tournament: {}.", tournament.unwrap_err()),
            });
        }

        Ok(tournament.unwrap())
    }

    /// Finds and returns all [`OsuTournament`] that match the filter.
    pub async fn find_tournaments(&self, filter: Document) -> RepoResult<Vec<OsuTournament>> {
        let cursor = self
            .tournaments
            .find(Some(filter), None)
            .await;

        if cursor.is_err() {
            return Err(RepoError::QueryFatal {
                message: format!("finding tournaments: {}.", cursor.unwrap_err()),
            });
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
        slug: String,
        title: String,
    ) -> Result<ObjectId, RepoError> {
        let tournament = self.find_tournament_by_id_or_slug(&slug).await;

        if tournament.is_ok() && tournament.unwrap().is_some() {
            return Err(RepoError::AlreadyExist {
                key: "tournament.slug".to_string(),
            });
        }

        let query_result = self
            .tournaments
            .clone_with_type::<Document>()
            // TODO: Using model instead of document
            .insert_one(
                doc! {
                    "slug": slug,
                    "title": title,
                },
                None,
            )
            .await;

        if query_result.is_err() {
            return Err(RepoError::QueryFatal {
                message: format!("creating tournament: {}.", query_result.unwrap_err()),
            });
        }

        let query_result = query_result.unwrap();

        Ok(query_result.inserted_id.as_object_id().unwrap())
    }

    // TODO: Ameliorate this
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
                        { "_id": ObjectId::parse_str(id_or_slug).unwrap_or_default() },
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

    /// Deletes the [`OsuTournament`] that matches the id or slug.
    pub async fn delete_tournament(&self, id_or_slug: &str) -> RepoResult<OsuTournament> {
        let query_result = self
            .tournaments
            .find_one_and_delete(
                doc! {
                    "$or": [
                        { "_id": ObjectId::parse_str(id_or_slug).unwrap_or_default() },
                        { "slug": id_or_slug }
                    ]
                },
                None,
            )
            .await;

        if query_result.is_err() {
            return Err(RepoError::QueryFatal {
                message: format!("deleting tournament: {}.", query_result.unwrap_err()),
            });
        }

        let query_result = query_result.unwrap();

        Ok(query_result)
    }
}
