use async_trait::async_trait;
use sqlx::{query, Pool, Postgres};

#[async_trait]
pub trait UserRepository {
    async fn get_user_by_email(&self, email: String);
}

struct PostgresUserRepository {
    db: Pool<Postgres>,
}

impl PostgresUserRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn get_user_by_email(&self, email: String) {
        let existing_user = query!(
            r#"
select *
from users
where email = $1::varchar
        "#,
            email
        )
        .fetch_optional(&self.db)
        .await
        .unwrap();
    }
}
