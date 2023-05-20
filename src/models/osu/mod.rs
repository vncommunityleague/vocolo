use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

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
    #[strum(props(display = "NoMod"))]
    NM,
    #[strum(props(display = "Hidden"))]
    HD,
    #[strum(props(display = "HardRock"))]
    HR,
    #[strum(props(display = "DoubleTime"))]
    DT,
    #[strum(props(display = "FreeMod"))]
    FM,
    #[strum(props(display = "Easy"))]
    EZ,
    #[strum(props(display = "HalfTime"))]
    HT,
    #[strum(props(display = "Flashlight"))]
    FL,
    #[strum(props(display = "Tiebreaker"))]
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
