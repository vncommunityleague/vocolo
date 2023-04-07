pub mod auth;
pub mod constants;

pub async fn split_string_to_list(string: String, separator: &str) -> Vec<String> {
    let mut list = Vec::new();

    for s in string.split(separator) {
        list.push(s.to_string());
    }

    list
}
