use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};

use conduit_core::articles::repository::{ArticlesRepository, GetArticleQuery, UpsertArticleQuery};

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
        query_as!(
            UpsertArticleQuery,
            r#"
    with inserted_article_cte as (
        insert into articles (created_at, updated_at, title, body, slug, description, user_id)
        values (current_timestamp, current_timestamp, $1::varchar, $2::varchar, $3::varchar, $4::varchar, $5)
        returning id as "id",
                  created_at as "created_at",
                  updated_at as "updated_at",
                  title as "title",
                  body as "body",
                  slug as "slug",
                  description as "description",
                  user_id as "user_id"
    ) select a.id as "id!",
           a.created_at as "created_at!",
           a.updated_at as "updated_at!",
           a.title as "title!",
           a.body as "body!",
           a.slug as "slug!",
           a.description as "description!",
           u.username as "author_username!",
           u.bio as "author_bio!",
           u.image as "author_image!"
    from inserted_article_cte a
    join users u on u.id = a.user_id
            "#,
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
        query_as!(
            UpsertArticleQuery,
            r#"
    with updated_article_cte as (
        update articles
        set updated_at = current_timestamp,
            title = $1::varchar,
            slug = $2::varchar,
            description = $3::varchar,
            body = $4::varchar
        where id = $5
        returning *
    ) select a.id as "id!",
             a.created_at as "created_at!",
             a.updated_at as "updated_at!",
             a.title as "title!",
             a.body as "body!",
             a.slug as "slug!",
             a.description as "description!",
             u.username as "author_username!",
             u.bio as "author_bio!",
             u.image as "author_image!"
    from updated_article_cte a
    join users u on u.id = a.user_id
            "#,
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
        query_as!(
            GetArticleQuery,
            r#"
        select a.id as "id!",
               a.created_at as "created_at!",
               a.updated_at as "updated_at!",
               a.title as "title!",
               a.body as "body!",
               a.description as "description!",
               a.slug as "slug!",
               u.id as "user_id!",
               exists(select 1 from article_favorites af where af.user_id = $1 and af.article_id = a.id) as "favorited!",
               (select count(*) from article_favorites where article_id = a.id) as "favorites!",
               exists(select 1 from user_follows where followee_id = a.user_id and follower_id = $1) "following_author!",
               u.username as "author_username!",
               u.bio as "author_bio!",
               u.image as "author_image!"
        from articles a
        join users u on u.id = a.user_id
        -- filter on users for the author
        where ($2::varchar is null or $2::varchar = u.username)
        -- filter on tags, if applicable
        and ($3::varchar is null or exists(
            select 1 from tags t
            join article_tags at on (t.id, a.id) = (at.tag_id, at.article_id)
            where tag = $3::varchar
        ))
        -- filter on the favoriting user
        and ($4::varchar is null or exists(
            select 1 from users favoriting_user
            join article_favorites f on favoriting_user.id = f.user_id
            where favoriting_user.username = $4::varchar)
        )
        order by a.created_at desc
        limit $5
        offset $6
            "#,
            user_id,
            author,
            tag,
            favorited,
            limit,
            offset)
            .fetch_all(&self.pool)
            .await
            .context("an unexpected error occured retrieving articles")
    }

    async fn get_article_by_slug(
        &self,
        user_id: Option<i64>,
        slug: String,
    ) -> anyhow::Result<Option<GetArticleQuery>> {
        query_as!(
            GetArticleQuery,
            r#"
        select a.id as "id!",
               a.created_at as "created_at!",
               a.updated_at as "updated_at!",
               a.title as "title!",
               a.body as "body!",
               a.description as "description!",
               a.slug as "slug!",
               u.id as "user_id!",
               exists(select 1 from article_favorites af where af.user_id = $1 and af.article_id = a.id) as "favorited!",
               (select count(*) from article_favorites where article_id = a.id) as "favorites!",
               exists(select 1 from user_follows where followee_id = a.user_id and follower_id = $1) "following_author!",
               u.username as "author_username!",
               u.bio as "author_bio!",
               u.image as "author_image!"
        from articles a
        join users u on u.id = a.user_id
        where a.slug = $2::varchar
            "#,
            user_id,
            slug)
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
}
