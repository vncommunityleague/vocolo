use mongodb::Client;

use crate::util::constants::EnvironmentVariable;

pub mod osu;
pub mod user;

#[derive(Clone)]
pub struct Repo {
    pub user: user::UserRepo,
    pub osu: osu::OsuRepo,
}

impl Repo {
    pub async fn init() -> Self {
        let client = Client::with_uri_str(EnvironmentVariable::MongoUri.value())
            .await
            .expect("Cannot connect to MongoDB");

        let user = user::UserRepo::init(&client).await;
        let osu = osu::OsuRepo::init(&client).await;
        Repo { user, osu }
    }
}
