use mongodb::Client;

use crate::util::constants::Database;

pub mod matches;
pub mod tournaments;

#[derive(Clone)]
pub struct OsuRepo {
    pub tournaments: tournaments::OsuTournamentRepo,
    pub matches: matches::OsuMatchRepo,
}

impl OsuRepo {
    pub async fn init(client: &Client) -> Self {
        let tournaments =
            tournaments::OsuTournamentRepo::init(&client.database(&Database::Osu.to_string()))
                .await;
        let matches =
            matches::OsuMatchRepo::init(&client.database(&Database::Osu.to_string())).await;

        OsuRepo {
            tournaments,
            matches,
        }
    }
}
