use std::collections::HashMap;

use serde::Deserialize;

pub mod authentication_service;
pub mod profile_service;

#[derive(Deserialize)]
pub struct Errors {
    pub error: HashMap<String, Vec<String>>,
}
