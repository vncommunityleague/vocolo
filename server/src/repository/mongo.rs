use mongodb::{bson::doc, Client, Collection};

use common::models::{osu::OsuPlayer, user::User};

pub struct UserRepo {
    col: Collection<User>,
}

impl UserRepo {
    pub async fn init(client: &Client) -> Self {
        let db = client.database("VCL");
        let col: Collection<User> = db.collection("User");
        UserRepo { col }
    }

    pub async fn find_by_osu_id(&self, id: String) -> Option<User> {
        let query_result = self
            .col
            .find_one(Some(doc! { "osu_id": id }), None)
            .await
            .expect(&format!("Cannot find user with osua! id."));
        query_result
    }

    pub async fn find_by_discord_id(&self, id: String) -> Option<User> {
        let query_result = self
            .col
            .find_one(Some(doc! { "discord_id": id }), None)
            .await
            .expect(&format!("Cannot find user with Discord id."));
        query_result
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

    pub async fn find_by_osu_id(&self, id: String) -> Option<OsuPlayer> {
        let query_result = self
            .col
            .find_one(Some(doc! { "osu_id": id }), None)
            .await
            .expect(&format!("Cannot find user with osua! id."));
        query_result
    }
}
