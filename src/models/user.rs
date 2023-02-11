use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<ObjectId>,
    pub discord_id: String,
}

pub struct Role {

}

pub struct Permission {
    
}
