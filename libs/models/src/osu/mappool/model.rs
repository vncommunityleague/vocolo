use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OsuMappool {
    pub id: i32,
    pub name: String,
}
