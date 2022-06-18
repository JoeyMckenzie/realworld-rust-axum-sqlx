use anyhow::Context;
use async_trait::async_trait;
use lazy_static::lazy_static;
use sqlx::postgres::PgRow;
use sqlx::{query_as, QueryBuilder, Row};

use conduit_core::tags::repository::{ArticleTagQuery, TagEntity, TagsRepository};

use crate::connection_pool::ConduitConnectionPool;

lazy_static! {
    static ref PG_CURRENT_TIMESTAMP: &'static str = "current_timestamp";
}

#[derive(Clone)]
pub struct PostgresTagsRepository {
    pool: ConduitConnectionPool,
}

impl PostgresTagsRepository {
    pub fn new(pool: ConduitConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagsRepository for PostgresTagsRepository {
    async fn get_tags(&self, tags: Vec<String>) -> anyhow::Result<Vec<TagEntity>> {
        let mut query_builder = QueryBuilder::new("select id, tag, created_at from tags ");

        if !tags.is_empty() {
            query_builder
                .push("where tag = any(")
                .push_bind(tags)
                .push(")");
        }

        query_builder
            .push("order by tag")
            .build()
            .map(|row: PgRow| TagEntity {
                id: row.get(0),
                tag: row.get(1),
                created_at: row.get(2),
            })
            .fetch_all(&self.pool)
            .await
            .context("an unexpected error occurred while retrieving tags")
    }

    async fn create_tags(&self, tags: Vec<String>) -> anyhow::Result<Vec<TagEntity>> {
        let mut query_builder = QueryBuilder::new("insert into tags (tag, created_at) ");

        query_builder.push_values(tags, |mut builder, tag| {
            builder.push_bind(tag).push(*PG_CURRENT_TIMESTAMP);
        });

        query_builder
            .push("returning *")
            .build()
            .map(|row: PgRow| TagEntity {
                id: row.get(0),
                tag: row.get(1),
                created_at: row.get(2),
            })
            .fetch_all(&self.pool)
            .await
            .context("an unexpected error occurred while creating article tags")
    }

    async fn get_article_tags_by_article_id(
        &self,
        article_id: i64,
    ) -> anyhow::Result<Vec<ArticleTagQuery>> {
        query_as!(
            ArticleTagQuery,
            r#"
        select at.id,
               at.article_id,
               at.tag_id,
               t.tag
        from article_tags at
        join tags t on t.id = at.article_id
        where article_id = $1
        order by t.tag
            "#,
            article_id
        )
        .fetch_all(&self.pool)
        .await
        .context("an unexpected error occurred while retrieving tags")
    }

    async fn get_article_tags_article_ids(
        &self,
        article_ids: Vec<i64>,
    ) -> anyhow::Result<Vec<ArticleTagQuery>> {
        query_as!(
            ArticleTagQuery,
            r#"
        select at.id as "id!",
               at.tag_id as "tag_id!",
               at.article_id as "article_id!",
               t.tag as "tag!"
        from article_tags at
        join tags t on t.id = at.tag_id
        where article_id = any($1)
        order by t.tag
            "#,
            article_ids.as_slice()
        )
        .fetch_all(&self.pool)
        .await
        .context("an unexpected error occurred while retrieving tags")
    }

    async fn create_article_tags(&self, tags: Vec<(i64, i64)>) -> anyhow::Result<()> {
        let mut query_builder =
            QueryBuilder::new("insert into article_tags (tag_id, article_id, created_at) ");

        query_builder.push_values(tags, |mut builder, (tag_id, article_id)| {
            builder
                .push_bind(tag_id)
                .push_bind(article_id)
                .push(*PG_CURRENT_TIMESTAMP);
        });

        query_builder
            .build()
            .execute(&self.pool)
            .await
            .context("an unexpected error occurred while creating article tags")?;

        Ok(())
    }
}
