use mongodb::Client;

use crate::util::constants::Database;

pub mod tournaments;

#[derive(Clone)]
pub struct OsuRepo {
    pub tournaments: tournaments::OsuTournamentRepo,
}

impl OsuRepo {
    pub async fn init(client: &Client) -> Self {
        let tournaments =
            tournaments::OsuTournamentRepo::init(&client.database(&Database::Osu.to_string()))
                .await;

        OsuRepo { tournaments }
    }
}
