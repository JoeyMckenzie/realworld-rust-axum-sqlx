use serde::{Deserialize, Serialize};

pub mod responses;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ProfileDto {
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}
