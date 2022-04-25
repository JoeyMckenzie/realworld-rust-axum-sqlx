use std::time;

pub struct User {
    pub id: String,
    pub created_at: time::Duration,
    pub updated_at: time::Duration,
    pub username: String,
    pub email: String,
    pub password: String,
}
