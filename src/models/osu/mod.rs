use std::str::FromStr;

use serde::{Deserialize, Serialize};

use derive_more::Display;

pub mod tournaments;

#[derive(Serialize, Deserialize, Clone)]
pub enum GameMode {
    Standard,
    Taiko,
    Catch,
    Mania,
}

#[derive(Display, Serialize, Deserialize, Clone, Debug)]
pub enum BeatmapMod {
    #[display(fmt = "NoMod")]
    NM,
    #[display(fmt = "Hidden")]
    HD,
    #[display(fmt = "HardRock")]
    HR,
    #[display(fmt = "DoubleTime")]
    DT,
    #[display(fmt = "FreeMod")]
    FM,
    #[display(fmt = "Easy")]
    EZ,
    #[display(fmt = "HalfTime")]
    HT,
    #[display(fmt = "Flashlight")]
    FL,
    #[display(fmt = "Tiebreaker")]
    TB,
}

impl FromStr for BeatmapMod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NM" => Ok(BeatmapMod::NM),
            "HD" => Ok(BeatmapMod::HD),
            "HR" => Ok(BeatmapMod::HR),
            "DT" => Ok(BeatmapMod::DT),
            "FM" => Ok(BeatmapMod::FM),
            "EZ" => Ok(BeatmapMod::EZ),
            "HT" => Ok(BeatmapMod::HT),
            "FL" => Ok(BeatmapMod::FL),
            "TB" => Ok(BeatmapMod::TB),
            _ => Err(()),
        }
    }
}

pub enum ScoreType {}
