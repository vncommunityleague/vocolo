use serde::{Deserialize, Serialize};

mod mappool;
pub use mappool::*;
mod mappool_ops;
pub use mappool_ops::*;
mod matchup;
pub use matchup::*;
mod matchup_ops;
pub use matchup_ops::*;
mod tournament;
pub use tournament::*;
mod tournament_ops;
pub use tournament_ops::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameMode {
    Standard,
    Mania,
}
