use std::env;
use std::str::FromStr;

#[allow(non_camel_case_types)] // Seriously, this is annoying.
#[derive(strum_macros::Display)]
pub enum EnvironmentVariable {
    // Server
    SERVER_PUBLIC_URL,
    SERVER_HOST,
    SERVER_PORT,

    // MongoDB
    MONGO_URI,

    // Osu
    OSU_CLIENT_ID,
    OSU_CLIENT_SECRET,

    // Discord
    DISCORD_CLIENT_ID,
    DISCORD_CLIENT_SECRET,
    DISCORD_BOT_TOKEN,
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
