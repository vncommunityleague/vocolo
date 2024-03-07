use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use vocolo_entity::osu_tournament::Model;

#[derive(Deserialize, Serialize)]
pub struct APIOsuTournament {
    pub id: i32,
    pub slug: String,
    pub name: String,

    pub start_date: DateTime,
    pub end_date: DateTime,

    pub registration_start_date: DateTime,
    pub registration_end_date: DateTime,
}

impl From<Model> for APIOsuTournament {
    fn from(tournament: Model) -> APIOsuTournament {
        APIOsuTournament {
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
