use mongodb::{Client, Collection, Database};
use crate::models::osu::tournaments::{OsuMappool, OsuMatch, OsuTournament};
use crate::util;

#[derive(Clone)]
pub struct OsuRepo {
    pub tournaments: OsuTournamentRepo,
}

impl OsuRepo {
    pub async fn init(client: &Client) -> Self {
        let db = &client.database(&util::constants::Database::Osu.to_string());
        let tournaments = OsuTournamentRepo::init(db).await;

        OsuRepo { tournaments }
    }
}

#[derive(Clone)]
pub struct OsuTournamentRepo {
    pub tournaments_col: Collection<OsuTournament>,
    pub mappools_col: Collection<OsuMappool>,
    pub matches_col: Collection<OsuMatch>,
}

impl OsuTournamentRepo {
    pub(crate) async fn init(database: &Database) -> Self {
        let tournaments_col: Collection<OsuTournament> = database.collection("tournaments");
        let mappools_col: Collection<OsuMappool> = database.collection("mappools");
        let matches_col: Collection<OsuMatch> = database.collection("matches");

        OsuTournamentRepo {
            tournaments_col,
            mappools_col,
            matches_col,
        }
    }
}
