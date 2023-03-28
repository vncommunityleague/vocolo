use mongodb::{Client, Collection};
use mongodb::bson::doc;
use common::models::user::User;

#[derive(Clone)]
pub struct UserRepo {
    user_col: Collection<User>,
}

impl UserRepo {
    pub async fn init(client: &Client) -> Self {
        let db = client.database("VCL");
        let user_col: Collection<User> = db.collection("User");
        UserRepo { user_col }
    }

    pub async fn find_by_osu_id(&self, id: &String) -> Option<User> {
        self.user_col
            .find_one(Some(doc! { "osu_id": id }), None)
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_discord_id(&self, id: &String) -> Option<User> {
        self.user_col
            .find_one(Some(doc! { "discord_id": id }), None)
            .await
            .unwrap_or(None)
    }
}
