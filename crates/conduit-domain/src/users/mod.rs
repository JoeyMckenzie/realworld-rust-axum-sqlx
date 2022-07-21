use serde::{Deserialize, Serialize};

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i64,
    pub username: String,
    pub email: String,
    pub bio: String,
    pub image: String,
    pub token: String,
}
