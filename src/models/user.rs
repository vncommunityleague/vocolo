use crate::models::ModelAttribute;
use crate::repository::model::ModelExt;
use crate::repository::RepoResult;
use async_trait::async_trait;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::Collection;
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


#[async_trait]
impl ModelExt for User {
    type T = User;

    async fn find_by_id(col: Collection<Self::T>, id: &ObjectId) -> RepoResult<Option<Self::T>> {
        let hex_id = id.to_hex();

        Self::find_one(col, doc! {
                "$or": [
                    { "_id": id },
                    { "discord_id": hex_id },
                    { "osu_id": hex_id },
                ]
            }, None
        ).await
    }
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
