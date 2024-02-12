use serde::{Deserialize, Serialize};

#[derive(garde::Validate, Serialize, Deserialize)]
#[garde(allow_unvalidated)]
pub struct UserCreation {
    // Auth ID, provided by Ory
    pub id: String,
    pub username: String,
}
