use async_trait::async_trait;
use std::sync::Arc;
use conduit_domain::users::models::User;


pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[async_trait]
pub trait UsersRepository {
    async fn get_user_by_email_or_username(
        &self,
        email: String,
        username: String,
    ) -> anyhow::Result<User>;
}
