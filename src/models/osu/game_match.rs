use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct APIOsuMatch {
    pub id: i32,
    pub name: String,
}