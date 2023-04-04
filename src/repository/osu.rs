use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::{bson::doc, Client, Collection};
use tokio_stream::StreamExt;

use crate::models::osu::OsuTournament;
use crate::models::{osu::OsuPlayer, user::User};

#[derive(Clone)]
pub struct OsuRepo {
    players: Collection<OsuPlayer>,
    tournaments: Collection<OsuTournament>,
}

impl OsuRepo {
    pub async fn init(client: &Client) -> Self {
        let db = client.database("vcl_osu");
        let players: Collection<OsuPlayer> = db.collection("OsuUser");
        let tournaments: Collection<OsuTournament> = db.collection("tournaments");

        OsuRepo {
            players,
            tournaments,
        }
    }

    pub async fn find_by_osu_id(&self, id: String) -> Option<OsuPlayer> {
        let query_result = self
            .players
            .find_one(Some(doc! { "osu_id": id }), None)
            .await
            .expect(&format!("Cannot find user with osu! id."));
        query_result
    }

    pub async fn find_tournament_by_slug(&self, slug: String) -> Option<OsuTournament> {
        let query_result = self
            .tournaments
            .find_one(Some(doc! { "slug": slug }), None)
            .await
            .expect(&format!("Cannot find tournament with provided slug."));
        query_result
    }

    pub async fn find_tournaments(&self) -> Option<Vec<OsuTournament>> {
        let mut cursor = self
            .tournaments
            .find(None, None)
            .await
            .expect(&format!("Cannot list tournaments."));
        let mut tournaments = Vec::new();
        while let Some(result) = cursor.next().await {
            tournaments.push(result.expect(&format!("Cannot list tournaments.")));
        }
        Some(tournaments)
    }

    pub async fn create_tournament(&self, slug: String, title: String) -> Option<ObjectId> {
        let query_result = self
            .tournaments
            .clone_with_type::<Document>()
            .insert_one(
                doc! {
                    "slug": slug,
                    "title": title,
                },
                None,
            )
            .await
            .expect(&format!("Cannot create tournament."));
        query_result.inserted_id.as_object_id()
    }
}
