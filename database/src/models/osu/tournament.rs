use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};
use revolt_optional_struct::OptionalStruct;
use serde::{Deserialize, Serialize};

use vocolo_internal::*;

use crate::Database;

use super::{AbstractOsuTournament, GameMode};

#[derive(OptionalStruct, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[optional_derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
#[optional_name = "PartialTournament"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct Tournament {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub slug: String,
    pub name: String,

    pub mode: GameMode,

    #[serde(skip_serializing_if = "vocolo_models::if_false", default)]
    pub invite_only: bool,
    pub min_team_size: i16,
    pub max_team_size: i16,

    pub registration_start_date: DateTime<Utc>,
    pub registration_end_date: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TournamentTeam>>,
}

#[derive(OptionalStruct, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[optional_derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
#[optional_name = "PartialTeam"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct TournamentTeam {
    pub id: ObjectId,
    pub name: String,

    pub captain: i32,
    pub players: Vec<i32>,
}

#[derive(OptionalStruct, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[optional_derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
#[optional_name = "PartialStaff"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct TournamentStaff {
    pub id: i32,
    pub role: TournamentStaffRole,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum TournamentStaffRole {
    Organizer,

    Mappooler,
    Mapper,
    Testplayer,

    Referee,
    Streamer,
    Commentator,

    Staff,
    Designer,
    Developer,
}

impl Tournament {
    pub async fn fetch(db: &Database, tournament_id: &str) -> Result<Tournament> {
        let tournament = db
            .fetch_osu_tournament(tournament_id)
            .await?
            .ok_or(Error::UnknownTournament)?;
        Ok(tournament)
    }

    pub async fn create(&self, db: &Database) -> Result<ObjectId> {
        if db
            .check_osu_tournament_exists(doc! { "slug": &self.slug })
            .await?
        {
            return Err(Error::AlreadyExists(
                "duplication on field slug".to_string(),
            ));
        }

        let id = db.insert_osu_tournament(self).await?;
        Ok(id)
    }

    pub async fn update(
        db: &Database,
        tournament_id: &str,
        partial: &PartialTournament,
    ) -> Result<()> {
        if partial.slug.is_some()
            && db
                .check_osu_tournament_exists(doc! { "slug": &partial.slug })
                .await?
        {
            return Err(Error::AlreadyExists(
                "duplication on field slug".to_string(),
            ));
        }

        db.update_osu_tournament(tournament_id, partial)
            .await?
            .ok_or(Error::UnknownTournament)?;
        Ok(())
    }

    pub async fn delete(db: &Database, tournament_id: &str) -> Result<()> {
        db.delete_osu_tournament(tournament_id)
            .await?
            .ok_or(Error::UnknownTournament)?;
        Ok(())
    }

    pub async fn get_teams(db: &Database, tournament_id: &str) -> Result<Vec<TournamentTeam>> {
        let teams = db
            .fetch_osu_tournament_teams(&tournament_id)
            .await?
            .ok_or(Error::UnknownTournament)?;
        Ok(teams)
    }

    pub async fn register_team(
        db: &Database,
        tournament_id: &str,
        team: TournamentTeam,
    ) -> Result<TournamentTeam> {
        let tournament = Self::fetch(db, tournament_id).await?;
        let now = Utc::now();

        if tournament.registration_end_date < now {
            return Err(Error::RegistrationClosed);
        } else if tournament.registration_start_date > now {
            return Err(Error::RegistrationNotOpen);
        }

        let duplicate_players = db
            .check_osu_tournament_players_exist(tournament_id, &team.players)
            .await?;

        if duplicate_players.len() > 0 {
            return Err(Error::AlreadyRegistered);
        }

        db.insert_osu_tournament_teams(tournament_id, vec![&team])
            .await?;

        Ok(team)
    }
}

impl TournamentStaffRole {
    pub fn not_allow_to_play(&self) -> bool {
        matches!(
            self,
            TournamentStaffRole::Organizer
                | TournamentStaffRole::Mappooler
                | TournamentStaffRole::Mapper
                | TournamentStaffRole::Testplayer
                | TournamentStaffRole::Referee
                | TournamentStaffRole::Designer
                | TournamentStaffRole::Developer
        )
    }
}
