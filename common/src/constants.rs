use std::env;
use std::str::FromStr;

#[derive(strum_macros::Display)]
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
    DiscordBotToken,
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

#[derive(strum_macros::Display)]
pub enum GameMode {
    Standard,
    Taiko,
    Catch,
    Mania,
}
