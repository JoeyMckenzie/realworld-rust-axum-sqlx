use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;

use conduit_core::articles::repository::{ArticlesRepository, CreateArticleQuery, GetArticleQuery};

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
    ) -> anyhow::Result<CreateArticleQuery> {
        query_as!(
            CreateArticleQuery,
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
               exists(select 1 from article_favorites af where af.user_id = $1 and af.article_id = a.id) as "favorited!",
               (select count(*) from article_favorites where article_id = a.id) as "favorites!",
               exists(select 1 from user_follows where followee_id = a.user_id and follower_id = $1) "following_author!",
               u.username as "author_username!",
               u.bio as "author_bio!",
               u.image as "author_image!"
        from articles a
        join users u on u.id = a.user_id
        join article_tags at on at.article_id = a.id
        join tags t on t.id = at.tag_id
        join article_favorites af on a.id = af.article_id
        where $2::varchar is null or $3::varchar = t.tag
        and $3::varchar is null or $3::varchar = u.username -- filter on users for the author
        and $4::varchar is null or $3::varchar = u.username -- filter again on users for the username, in theory this only returns zero or one given an author, consider it user error
        limit $5
        offset $6
            "#,
            user_id,
            tag,
            author,
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
        slug: &str,
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
               exists(select 1 from article_favorites af where af.user_id = $1 and af.article_id = a.id) as "favorited!",
               (select count(*) from article_favorites where article_id = a.id) as "favorites!",
               exists(select 1 from user_follows where followee_id = a.user_id and follower_id = $1) "following_author!",
               u.username as "author_username!",
               u.bio as "author_bio!",
               u.image as "author_image!"
        from articles a
        join users u on u.id = a.user_id
        join article_tags at on at.article_id = a.id
        join tags t on t.id = at.tag_id
        join article_favorites af on a.id = af.article_id
        where a.slug = $2::varchar
            "#,
            user_id,
            slug)
            .fetch_optional(&self.pool)
            .await
            .context("an unexpected error occured retrieving articles")
    }
}
