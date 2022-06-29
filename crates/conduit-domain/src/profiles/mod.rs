use serde::{Deserialize, Serialize};

pub mod responses;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileDto {
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}
