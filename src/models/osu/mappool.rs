use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct APIOsuMappool {
    pub id: i32,
    pub name: String,
}
