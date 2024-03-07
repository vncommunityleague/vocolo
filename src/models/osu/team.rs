use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct APIOsuTeam {
    pub id: i32,
    pub name: String,

    pub captain: i32,
    pub members: Vec<i32>,
}
