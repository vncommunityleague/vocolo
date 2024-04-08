use crate::database::Database;

pub mod osu;

pub trait AbstractDatabase:
    Sync + Send + osu::AbstractOsuTournament + osu::AbstractOsuMatchup + osu::AbstractOsuMappool
{
}

impl AbstractDatabase for Database {}
