use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use vocolo_entity::user::Model;

#[derive(Deserialize, Serialize)]
pub struct APIUser {
    pub id: i32,
    pub identity_id: Uuid,
}

impl From<Model> for APIUser {
    fn from(user: Model) -> Self {
        Self {
            id: user.id,
            identity_id: user.identity_id,
        }
    }
}
