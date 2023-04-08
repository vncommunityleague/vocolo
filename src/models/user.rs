use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone)]
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

impl User {
    pub async fn is_admin(&self) -> bool {
        self.roles.contains(&Role::ADMIN) || self.roles.contains(&Role::DEVELOPER)
    }

    pub async fn is_moderator(&self) -> bool {
        self.roles.contains(&Role::MODERATOR) || self.is_admin().await
    }

    pub async fn has_role(&self, role: Role) -> bool {
        self.roles.contains(&role) || self.is_moderator().await
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StaffUser {
    pub role: Role
}
