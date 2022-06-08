use std::sync::Arc;

use crate::errors::ConduitResult;

/// A security service for handling JWT authentication.
pub type DynSecurityService = Arc<dyn SecurityService + Send + Sync>;

pub trait SecurityService {
    fn hash_password(&self, raw_password: &str) -> ConduitResult<String>;
    fn verify_password(
        &self,
        stored_password: &str,
        attempted_password: &str,
    ) -> ConduitResult<bool>;
}
