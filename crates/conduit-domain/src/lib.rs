use serde::{Deserialize, Serialize};

pub mod articles;
pub mod comments;
pub mod profiles;
pub mod tags;
pub mod users;

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct PingResponse {
    pub message: String,
}

impl PingResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Default for PingResponse {
    fn default() -> Self {
        Self::new(String::from("API is responsive"))
    }
}
