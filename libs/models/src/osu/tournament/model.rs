use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OsuTournament {
    pub id: i32,
    pub slug: String,
    pub name: String,

    pub start_date: DateTime,
    pub end_date: DateTime,

    pub registration_start_date: DateTime,
    pub registration_end_date: DateTime,
}
