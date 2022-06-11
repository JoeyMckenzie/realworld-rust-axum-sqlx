use serde::{Deserialize, Serialize};

pub mod responses;

#[derive(Serialize, Deserialize)]
pub struct ProfileDto {
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}
