use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

pub mod osu;
pub mod tournaments;
pub mod user;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Timestamp {
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

impl Default for Timestamp {
    fn default() -> Self {
        Self {
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
