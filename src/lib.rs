extern crate reqwest;
extern crate serde_json;
pub use reqwest::StatusCode;

mod bot;
mod message;
mod update;
mod error;
pub mod updater;

/// Construct an API URL with the base bot URL and an
/// array of strings which will construct the path.
fn construct_api_url(bot_url: &str, path: &[&str]) -> String {
    format!("{}/{}", bot_url, path.join("/"))
}

/// Get the String value from a JSON field
fn string_from_json(json: &serde_json::Value, field: &str) -> Option<String> {
    json.find(field).and_then(|v| v.as_str()).map(|s| s.to_owned())
}
