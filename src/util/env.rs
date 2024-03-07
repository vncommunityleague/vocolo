pub fn var(key: &str) -> String {
    std::env::var(key)
        .unwrap_or_else(|_| panic!("{} must be set", key))
        .trim()
        .to_string()
}

pub fn var_opt(key: &str) -> Option<String> {
    std::env::var(key).ok()
}
