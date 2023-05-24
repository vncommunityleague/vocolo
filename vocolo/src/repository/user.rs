use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};

use crate::models::user::User;
use crate::util::auth::AuthType;
use crate::util::constants::Database;

#[derive(Clone)]
pub struct UserRepo {
    pub user_col: Collection<User>,
}

impl UserRepo {
    pub async fn init(client: &Client) -> Self {
        let db = client.database(&Database::Main.to_string());
        let user_col: Collection<User> = db.collection("User");

        UserRepo { user_col }
    }
}
