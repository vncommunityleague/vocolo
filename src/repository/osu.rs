use crate::models::osu::tournaments::OsuTournament;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::{bson::doc, Client, Collection};
use tokio_stream::StreamExt;

use crate::util::constants::Database;

// TODO: Handle errors

#[derive(Clone)]
pub struct OsuRepo {
    pub tournaments: Collection<OsuTournament>,
}

impl OsuRepo {
    pub async fn init(client: &Client) -> Self {
        let db = client.database(Database::Osu.db_name());
        let tournaments: Collection<OsuTournament> = db.collection("tournaments.rs");

        OsuRepo { tournaments }
    }

    /// Lists all [`OsuTournament`] in the database.
    pub async fn list_tournaments(&self) -> Option<Vec<OsuTournament>> {
        self.find_tournaments(doc! {}).await
    }

    /// Finds the [`OsuTournament`] that matches the id or slug.
    pub async fn find_tournament_by_id_or_slug(&self, id_or_slug: &str) -> Option<OsuTournament> {
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
    ) -> Option<Vec<OsuTournament>> {
        self.find_tournaments(doc! {
            "$or": [
                { "_id": { "$in": id_or_slug_list.iter().map(|id| ObjectId::parse_str(id).unwrap_or_default()).collect::<Vec<ObjectId>>() } },
                { "slug": { "$in": id_or_slug_list } }
            ]
        }).await
    }

    /// Finds the [`OsuTournament`] that matches the id or slug.
    pub async fn find_tournament(&self, filter: Document) -> Option<OsuTournament> {
        self.tournaments
            .find_one(Some(filter), None)
            .await
            .unwrap_or_else(|t| panic!("Unexpected error while finding tournament {0}.", t))
    }

    /// Finds and returns all [`OsuTournament`] that match the filter.
    pub async fn find_tournaments(&self, filter: Document) -> Option<Vec<OsuTournament>> {
        let mut cursor = self
            .tournaments
            .find(Some(filter), None)
            .await
            .unwrap_or_else(|_| panic!("Unexpected error while finding tournaments.rs."));

        cursor.next().await.as_ref()?;

        let mut tournaments = Vec::new();

        while let Some(result) = cursor.next().await {
            tournaments.push(result.expect("null"));
        }

        Some(tournaments)
    }

    /// Creates a new [`OsuTournament`] and returns its id.
    pub async fn create_tournament(&self, slug: String, title: String) -> Option<ObjectId> {
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
            .await
            .unwrap_or_else(|_| panic!("Cannot create tournament."));
        query_result.inserted_id.as_object_id()
    }

    pub async fn replace_tournament(
        &self,
        id_or_slug: &str,
        new_data: OsuTournament,
    ) -> Option<OsuTournament> {
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
            .unwrap_or_else(|_| panic!("Cannot modify tournament."));

        if query_result.modified_count > 0 {
            self.find_tournament_by_id_or_slug(id_or_slug).await
        } else {
            None
        }
    }

    /// Deletes the [`OsuTournament`] that matches the id or slug.
    pub async fn delete_tournament(&self, id_or_slug: String) -> Option<OsuTournament> {
        let query_result = self
            .tournaments
            .find_one_and_delete(
                doc! {
                    "$or": [
                        { "_id": ObjectId::parse_str(&id_or_slug).unwrap_or_default() },
                        { "slug": id_or_slug }
                    ]
                },
                None,
            )
            .await
            .unwrap_or_else(|_| panic!("Cannot delete tournament."));

        if query_result.is_some() {
            Some(query_result.unwrap())
        } else {
            None
        }
    }
}
