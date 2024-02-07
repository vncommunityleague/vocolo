mod request;
pub mod response;

pub fn env_var(key: &str) -> String {
    dotenvy::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}

pub fn env_var_opt(key: &str) -> Option<String> {
    dotenvy::var(key).ok()
}
