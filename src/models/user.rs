use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Role {
    ADMIN,
    DEVELOPER,
    MODERATOR,
    GFX,
    REFEREE,
    CASTER,
    STREAMER,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub discord_id: String,
    pub osu_id: String,

    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StaffUser {
    pub user: ObjectId,
}
