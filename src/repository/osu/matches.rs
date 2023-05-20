use mongodb::{Collection, Database};

use crate::models::osu::tournaments::OsuMatch;

#[derive(Clone)]
pub struct OsuMatchRepo {
    pub matches: Collection<OsuMatch>,
}

impl OsuMatchRepo {
    pub(crate) async fn init(database: &Database) -> Self {
        let matches: Collection<OsuMatch> = database.collection("matches");

        OsuMatchRepo { matches }
    }
}
