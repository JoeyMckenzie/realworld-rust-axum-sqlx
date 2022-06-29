use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as, query_file_as};

use conduit_core::articles::repository::{
    ArticlesRepository, GetArticleFavoritesQuery, GetArticleQuery, UpsertArticleQuery,
};

use crate::connection_pool::ConduitConnectionPool;

#[derive(Clone)]
pub struct PostgresArticlesRepository {
    pool: ConduitConnectionPool,
}

impl PostgresArticlesRepository {
    pub fn new(pool: ConduitConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticlesRepository for PostgresArticlesRepository {
    async fn create_article(
        &self,
        user_id: i64,
        title: String,
        slug: String,
        description: String,
        body: String,
    ) -> anyhow::Result<UpsertArticleQuery> {
        query_file_as!(
            UpsertArticleQuery,
            "queries/insert_article.sql",
            title,
            body,
            slug,
            description,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured creating article")
    }

    async fn update_article(
        &self,
        id: i64,
        title: String,
        slug: String,
        description: String,
        body: String,
    ) -> anyhow::Result<UpsertArticleQuery> {
        query_file_as!(
            UpsertArticleQuery,
            "queries/update_article.sql",
            title,
            slug,
            description,
            body,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured creating article")
    }

    async fn get_articles(
        &self,
        user_id: Option<i64>,
        tag: Option<String>,
        author: Option<String>,
        favorited: Option<String>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<GetArticleQuery>> {
        query_file_as!(
            GetArticleQuery,
            "queries/get_articles.sql",
            user_id,
            author,
            tag,
            favorited,
            limit as i32,
            offset as i32
        )
        .fetch_all(&self.pool)
        .await
        .context("an unexpected error occured retrieving articles")
    }

    async fn get_article_by_slug(&self, user_id: Option<i64>, slug: String) -> anyhow::Result<Option<GetArticleQuery>> {
        query_file_as!(GetArticleQuery, "queries/get_article_by_slug.sql", user_id, slug)
            .fetch_optional(&self.pool)
            .await
            .context("an unexpected error occured retrieving articles")
    }

    async fn delete_article(&self, id: i64) -> anyhow::Result<()> {
        query!(
            r#"
    delete from articles
    where id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting article")?;

        Ok(())
    }

    async fn favorite_article(&self, article_id: i64, user_id: i64) -> anyhow::Result<GetArticleQuery> {
        query_as!(
            GetArticleQuery,
            r#"
        with favorited_article_cte as (
            insert into user_favorites (created_at, user_id, article_id)
            values (current_timestamp, $1, $2)
            returning *
        ) select a.id as "id!",
                 a.created_at as "created_at!",
                 a.updated_at as "updated_at!",
                 a.title as "title!",
                 a.body as "body!",
                 a.description as "description!",
                 a.slug as "slug!",
                 u.id as "user_id!",
                 true as "favorited!",
                 (select count(*) + 1 from user_favorites where article_id = a.id) as "favorites!",
                 exists(select 1 from user_follows where followee_id = a.user_id and follower_id = $1) "following_author!",
                 u.username as "author_username!",
                 u.bio as "author_bio!",
                 u.image as "author_image!"
        from favorited_article_cte fa
        join users u on fa.user_id = u.id
        join articles a on fa.article_id = a.id
        where a.id = $2
            "#,
            user_id,
            article_id
        )
            .fetch_one(&self.pool)
            .await
            .context("an unexpected error occurred while adding user favorite for the article")
    }

    async fn unfavorite_article(&self, article_id: i64, user_id: i64) -> anyhow::Result<GetArticleQuery> {
        query_as!(
            GetArticleQuery,
            r#"
        with unfavorited_article_cte as (
            delete from user_favorites
            where (user_id, article_id) = ($1, $2)
        ) select a.id as "id!",
                 a.created_at as "created_at!",
                 a.updated_at as "updated_at!",
                 a.title as "title!",
                 a.body as "body!",
                 a.description as "description!",
                 a.slug as "slug!",
                 u.id as "user_id!",
                 false as "favorited!",
                 (select count(*) - 1 from user_favorites where article_id = a.id) as "favorites!",
                 exists(select 1 from user_follows where followee_id = a.user_id and follower_id = $1) "following_author!",
                 u.username as "author_username!",
                 u.bio as "author_bio!",
                 u.image as "author_image!"
        from articles a
        join users u on a.user_id = u.id
        where a.id = $2
            "#,
            user_id,
            article_id
        )
            .fetch_one(&self.pool)
            .await
            .context("an unexpected error occurred while adding user favorite for the article")
    }

    async fn get_user_favorites(&self, article_id: i64) -> anyhow::Result<Vec<GetArticleFavoritesQuery>> {
        query_as!(
            GetArticleFavoritesQuery,
            r#"
        select id,
               article_id,
               user_id
        from user_favorites
        where article_id = $1::bigint
            "#,
            article_id
        )
        .fetch_all(&self.pool)
        .await
        .context("an unexpected error occured retrieving article favorites")
    }
}
