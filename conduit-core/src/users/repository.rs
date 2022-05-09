use argon2::Config;
use std::env::VarError;
use std::{env, time};

pub struct User {
    pub id: String,
    pub created_at: time::Duration,
    pub updated_at: time::Duration,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn hash_password(&self) -> Result<String, VarError> {
        let config = Config::default();
        let salt = env::var("PASSWORD_SALT").expect("port is not defined as an env var");

        Ok(argon2::hash_encoded(self.password.as_bytes(), salt.as_bytes(), &config).unwrap())
    }
}
