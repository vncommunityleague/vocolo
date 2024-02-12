use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use crate::user::{User, UserCreation};
use vocolo_database_entities::user::*;

pub struct UserHandler;

impl UserHandler {
    pub async fn create(
        db: &DatabaseConnection,
        data: UserCreation,
    ) -> crate::Result<User> {
        let user = ActiveModel {
            ..Default::default()
        };

        let user: Model = user.insert(db).await?;

        Ok(Self::_to_dto(user))
    }

    fn _to_dto(user: Model) -> User {
        User {
            id: user.id,
        }
    }
}