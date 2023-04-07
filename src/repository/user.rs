use crate::models::user::User;
use crate::util::auth::AuthType;
use crate::util::constants::Database;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};
use strum::EnumProperty;

#[derive(Clone)]
pub struct UserRepo {
    user_col: Collection<User>,
}

impl UserRepo {
    pub async fn init(client: &Client) -> Self {
        let db = client.database(Database::Main.get_str("db_name").unwrap());
        let user_col: Collection<User> = db.collection("User");
        UserRepo { user_col }
    }

    pub async fn create(&self, id: String, auth_type: AuthType) {
        self.user_col
            .clone_with_type::<Document>()
            .insert_one(
                doc! {
                    auth_type.repo_path(): id,
                },
                None,
            )
            .await
            .unwrap();
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
