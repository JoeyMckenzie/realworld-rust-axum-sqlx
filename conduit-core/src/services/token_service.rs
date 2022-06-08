use std::sync::Arc;

use crate::errors::ConduitResult;

/// A security service for handling JWT authentication.
pub type DynTokenService = Arc<dyn TokenService + Send + Sync>;

pub trait TokenService {
    fn new_token(&self, user_id: i64, email: &str) -> ConduitResult<String>;
}
