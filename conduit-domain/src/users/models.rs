use std::time;

pub struct User {
    pub id: u64,
    pub created_at: time::Duration,
    pub updated_at: time::Duration,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, Debug)]
pub struct UserDto {
    pub username: String,
    pub email: String,
    pub bio: String,
    pub image: String,
    pub token: String,
}
