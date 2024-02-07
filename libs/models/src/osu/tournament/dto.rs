use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

const MIN_SLUG_LENGTH: usize = 2;
const MAX_SLUG_LENGTH: usize = 8;
const MIN_NAME_LENGTH: usize = 4;
const MAX_NAME_LENGTH: usize = 64;

#[derive(garde::Validate, Serialize, Deserialize)]
#[garde(allow_unvalidated)]
pub struct OsuTournamentCreation {
    #[garde(length(min = MIN_SLUG_LENGTH, max = MAX_SLUG_LENGTH))]
    pub slug: String,
    #[garde(length(min = MIN_NAME_LENGTH, max = MAX_NAME_LENGTH))]
    pub name: String,
    pub start_date: Option<DateTime>,
    pub end_date: Option<DateTime>,
    pub registration_start_date: Option<DateTime>,
    pub registration_end_date: Option<DateTime>,
}

#[derive(garde::Validate, Serialize, Deserialize)]
#[garde(allow_unvalidated)]
pub struct OsuTournamentUpdate {
    #[garde(length(min = MIN_SLUG_LENGTH, max = MAX_SLUG_LENGTH))]
    pub slug: Option<String>,
    #[garde(length(min = MIN_NAME_LENGTH, max = MAX_NAME_LENGTH))]
    pub name: Option<String>,
    pub start_date: Option<DateTime>,
    pub end_date: Option<DateTime>,
    pub registration_start_date: Option<DateTime>,
    pub registration_end_date: Option<DateTime>,
}
