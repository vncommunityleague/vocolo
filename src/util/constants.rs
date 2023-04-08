use std::env;
use std::str::FromStr;

use derive_more::{Display};

#[derive(Display)]
pub enum Database {
    Main,
    Osu,
}

impl Database {
    pub fn db_name(&self) -> &str {
        match *self {
            Database::Main => "vcl",
            Database::Osu => "vcl_osu",
        }
    }
}

#[derive(Display)]
pub enum EnvironmentVariable {
    // Server
    ServerPublicUrl,
    ServerHost,

    // MongoDB
    MongoUri,

    // Osu
    OsuClientId,
    OsuClientSecret,

    // Discord
    DiscordClientId,
    DiscordClientSecret,
}

impl EnvironmentVariable {
    pub fn value(&self) -> String {
        env::var(self.to_string()).unwrap_or_else(|_| panic!("{} is missing or invalid.", self))
    }

    pub fn value_with_type<T: FromStr>(&self) -> T {
        match self.value().parse::<T>() {
            Ok(value) => value,
            Err(_) => panic!("{} is not suitable for this type", self),
        }
    }
}
