use serde::{Deserialize, Serialize};

pub use mappool::*;
pub use matchup::*;
pub use tournament::*;

mod mappool;
mod matchup;
mod tournament;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameMode {
    Standard,
    Mania,
}
