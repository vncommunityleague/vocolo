use mongodb::{Collection, Database};
use mongodb::bson::{doc, Document};

use crate::models::osu::tournaments::OsuMatch;
use crate::repository::{RepoError, RepoResult, to_object_id};

#[derive(Clone)]
pub struct OsuMatchRepo {
    pub matches: Collection<OsuMatch>,
}

impl OsuMatchRepo {
    pub(crate) async fn init(database: &Database) -> Self {
        let matches: Collection<OsuMatch> = database.collection("matches");

        OsuMatchRepo { matches }
    }

    pub async fn find_match_by_id(&self, id: &str) -> RepoResult<OsuMatch> {
        self.find_match(doc! {
            "_id": to_object_id(id)
        }).await
    }

    /// Finds the [`OsuMatch`] that matches the filter.
    pub async fn find_match(&self, filter: Document) -> RepoResult<OsuMatch> {
        let game_match = self.matches.find_one(Some(filter), None).await;

        return match game_match {
            Ok(v) => Ok(v),
            Err(e) => Err(RepoError::Internal(e)),
        };
    }
}
