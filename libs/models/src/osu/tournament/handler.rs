use chrono::Utc;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::*;

use vocolo_database_entities::osu_tournament::*;

use crate::error::Error;

use super::*;

pub struct OsuTournamentHandler;

impl OsuTournamentHandler {
    pub async fn create(
        db: &DatabaseConnection,
        data: OsuTournamentCreation,
    ) -> crate::Result<OsuTournament> {
        let tournament = ActiveModel {
            slug: Set(data.slug),
            name: Set(data.name),
            start_date: Set(data.start_date.unwrap_or(Utc::now().naive_utc())),
            end_date: Set(data.end_date.unwrap_or(Utc::now().naive_utc())),
            registration_start_date: Set(data
                .registration_start_date
                .unwrap_or(Utc::now().naive_utc())),
            registration_end_date: Set(data
                .registration_end_date
                .unwrap_or(Utc::now().naive_utc())),
            ..Default::default()
        };

        let tournament: Model = tournament.insert(db).await?;

        Ok(Self::_to_dto(tournament))
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> crate::Result<OsuTournament> {
        let tournament = Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(Error::Database)?
            .ok_or(Error::NotFound("osu_tournament".to_owned()))?;

        Ok(Self::_to_dto(tournament))
    }

    pub async fn get_by_slug(db: &DatabaseConnection, slug: &str) -> crate::Result<OsuTournament> {
        let tournament = Entity::find()
            .filter(Column::Slug.eq(slug))
            .one(db)
            .await
            .map_err(Error::Database)?
            .ok_or(Error::NotFound("osu_tournament".to_owned()))?;

        Ok(Self::_to_dto(tournament))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        data: OsuTournamentUpdate,
    ) -> crate::Result<OsuTournament> {
        let tournament = Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(Error::Database)?
            .ok_or(Error::NotFound("osu_tournament".to_owned()))?;

        let tournament = ActiveModel {
            id: Set(tournament.id),
            slug: Set(data.slug.unwrap_or(tournament.slug)),
            name: Set(data.name.unwrap_or(tournament.name)),
            start_date: Set(data.start_date.unwrap_or(tournament.start_date)),
            end_date: Set(data.end_date.unwrap_or(tournament.end_date)),
            registration_start_date: Set(data
                .registration_start_date
                .unwrap_or(tournament.registration_start_date)),
            registration_end_date: Set(data
                .registration_end_date
                .unwrap_or(tournament.registration_end_date)),
        }
        .update(db)
        .await
        .map_err(Error::Database)?;

        Ok(Self::_to_dto(tournament))
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> crate::Result<()> {
        let tournament = Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(Error::Database)?
            .ok_or(Error::NotFound("osu_tournament".to_owned()))?;

        tournament.delete(db).await.map_err(Error::Database)?;

        Ok(())
    }

    fn _to_dto(tournament: Model) -> OsuTournament {
        OsuTournament {
            id: tournament.id,
            slug: tournament.slug,
            name: tournament.name,
            start_date: tournament.start_date,
            end_date: tournament.end_date,
            registration_start_date: tournament.registration_start_date,
            registration_end_date: tournament.registration_end_date,
        }
    }
}
