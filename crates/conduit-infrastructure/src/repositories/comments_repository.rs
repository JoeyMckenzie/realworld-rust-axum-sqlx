use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};

use conduit_core::comments::repository::{CommentEntity, CommentQuery, CommentsRepository};

use crate::connection_pool::ConduitConnectionPool;

pub struct PostgresCommentsRepository {
    pool: ConduitConnectionPool,
}

impl PostgresCommentsRepository {
    pub fn new(pool: ConduitConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommentsRepository for PostgresCommentsRepository {
    async fn get_comments(&self, user_id: Option<i64>, article_id: i64) -> anyhow::Result<Vec<CommentQuery>> {
        query_as!(
            CommentQuery,
            r#"
        select c.id as "id!",
               c.body as "body!",
               c.created_at as "created_at!",
               c.updated_at as "updated_at!",
               u.username as "author_username!",
               u.bio as "author_bio!",
               u.image as "author_image!",
               exists(select 1 from user_follows uf where (
                   $1::bigint is null or (uf.followee_id, uf.follower_id) = ($1::bigint, c.user_id))
               ) as "following_author!"
        from comments c
        join users u on c.user_id = u.id
        where c.article_id = $2
            "#,
            user_id,
            article_id
        )
        .fetch_all(&self.pool)
        .await
        .context("an unexpected error occurred while retrieving comments")
    }

    async fn get_comment(&self, comment_id: i64) -> anyhow::Result<Option<CommentEntity>> {
        query_as!(
            CommentEntity,
            r#"
        select *
        from comments
        where id = $1::bigint
            "#,
            comment_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("an unexpected error occurred while creating comment")
    }

    async fn create_comment(&self, article_id: i64, user_id: i64, body: String) -> anyhow::Result<CommentQuery> {
        query_as!(
            CommentQuery,
            r#"
        with insert_comment_cte as (
            insert into comments (body, user_id, article_id, created_at, updated_at)
            values ($1::varchar, $2::bigint, $3::bigint, current_timestamp, current_timestamp)
            returning *
        ) select c.id as "id!",
                 c.body as "body!",
                 c.created_at as "created_at!",
                 c.updated_at as "updated_at!",
                 u.username as "author_username!",
                 u.bio as "author_bio!",
                 u.image as "author_image!",
                 false as "following_author!"
        from insert_comment_cte c
        join users u on c.user_id = u.id
        where c.article_id = $3::bigint
            "#,
            body,
            user_id,
            article_id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occurred while creating comment")
    }

    async fn delete_comment(&self, comment_id: i64) -> anyhow::Result<()> {
        query!(
            r#"
       delete from comments
       where id = $1
            "#,
            comment_id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred while deleting comment")?;

        Ok(())
    }
}
