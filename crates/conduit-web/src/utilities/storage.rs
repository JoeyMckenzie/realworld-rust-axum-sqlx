use gloo::storage::{LocalStorage, Storage};
use gloo_utils::window;
use lazy_static::lazy_static;
use log::{error, info};
use serde::{Deserialize, Serialize};

use super::errors::ConduitWebResult;

#[derive(Serialize, Deserialize)]
struct UserMeta {
    pub token: String,
}

impl UserMeta {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

lazy_static! {
    static ref TOKEN_STORAGE_KEY: &'static str = "userToken";
}

pub fn stash_token(token: String) -> ConduitWebResult<()> {
    info!("storing user metadata in local storage");
    let user_meta = UserMeta::new(token);
    LocalStorage::set(*TOKEN_STORAGE_KEY, user_meta)?;
    Ok(())
}

pub fn get_token() -> ConduitWebResult<String> {
    info!("retrieving stashed user metadata from local storage");
    let stored_meta: UserMeta = LocalStorage::get(*TOKEN_STORAGE_KEY)?;
    Ok(stored_meta.token)
}

pub fn clear_token() {
    info!("clearing stashed token");
    LocalStorage::delete(*TOKEN_STORAGE_KEY);
    if window().location().reload().is_err() {
        error!("window reload after clearing token failed");
    }
}
