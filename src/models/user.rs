use crate::models::ModelAttribute;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Role {
    /// Global roles
    /// Still can be used in tournaments
    Admin,
    Developer,
    Moderator,
    Gfx,
    Referee,
    Caster,
    Streamer,
    Spreadsheeter,

    Member,

    PlayTester,

    /// Tournament roles
    Host,

    /// Osu roles
    Mapper,
    Mappooler,
    // Others
}

impl Default for Role {
    fn default() -> Self {
        Role::Member
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(flatten)]
    pub model_attribute: ModelAttribute,

    pub discord_id: String,
    pub osu_id: String,

    pub roles: Vec<Role>,
}

impl User {
    pub async fn is_admin(&self) -> bool {
        self.roles.contains(&Role::Admin) || self.roles.contains(&Role::Developer)
    }

    pub async fn is_moderator(&self) -> bool {
        self.roles.contains(&Role::Moderator) || self.is_admin().await
    }

    pub async fn has_role(&self, role: Role) -> bool {
        self.roles.contains(&role) || self.is_moderator().await
    }
}
