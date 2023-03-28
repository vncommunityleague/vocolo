use mongodb::Client;
use common::constants::EnvironmentVariable;

pub mod osu;
pub mod user;

#[derive(Clone)]
pub struct Repo {
    pub user: user::UserRepo,
    pub osu: osu::OsuRepo,
}

impl Repo {
    pub async fn init() -> Self {
        let client = Client::with_uri_str(EnvironmentVariable::MONGO_URI.value())
            .await
            .expect("Cannot connect to MongoDB");

        let user = user::UserRepo::init(&client).await;
        let osu = osu::OsuRepo::init(&client).await;
        Repo { user, osu }
    }
}
