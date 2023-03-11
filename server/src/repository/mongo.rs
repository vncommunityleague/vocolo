use common::models::{osu::OsuPlayer, user::User};
use mongodb::{Client, Collection};

pub struct UserRepo {
    col: Collection<User>,
}

impl UserRepo {
    pub async fn init(client: &Client) -> Self {
        let db = client.database("VCL");
        let col: Collection<User> = db.collection("User");
        UserRepo { col }
    }
}

pub struct OsuRepo {
    col: Collection<OsuPlayer>,
}

impl OsuRepo {
    pub async fn init(client: &Client) -> Self {
        let db = client.database("VCL");
        let col: Collection<OsuPlayer> = db.collection("OsuUser");
        OsuRepo { col }
    }
}
