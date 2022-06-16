use async_trait::async_trait;
use itertools::Itertools;
use slug::slugify;
use tracing::info;

use conduit_core::articles::repository::DynArticlesRepository;
use conduit_core::articles::service::ArticlesService;
use conduit_core::errors::{ConduitError, ConduitResult};
use conduit_core::tags::repository::DynTagsRepository;
use conduit_domain::articles::models::ArticleDto;

pub struct ConduitArticlesService {
    articles_repository: DynArticlesRepository,
    tags_repository: DynTagsRepository,
}

impl ConduitArticlesService {
    pub fn new(
        articles_repository: DynArticlesRepository,
        tags_repository: DynTagsRepository,
    ) -> Self {
        Self {
            articles_repository,
            tags_repository,
        }
    }
}

#[async_trait]
impl ArticlesService for ConduitArticlesService {
    /// TODO: this flow should _really_ be handled within a transaction
    /// this is a lot harder to do than expected while hiding transaction details within
    /// the repositories themselves so our services can still maintain ease of testability
    /// see other other branches for their attempts, but I would _like_ to surround the repositories
    /// with a context/unit of work orchestrator but it seems much more complicated than anticipated
    async fn create_article(
        &self,
        user_id: i64,
        title: String,
        description: String,
        body: String,
        tag_list: Vec<String>,
    ) -> ConduitResult<ArticleDto> {
        // collect a unique list of the article tags to create
        let deduped_tag_list = tag_list.into_iter().unique().collect_vec();

        // search for existing tags, as we want to create a new tag if the request contains a tag that doesn't exist in the database
        let existing_tags = self
            .tags_repository
            .get_tags(deduped_tag_list.clone())
            .await?
            .into_iter()
            .map(|tag| tag.tag)
            .collect_vec();

        let mut tags_to_create: Vec<String> = Vec::new();

        // roll through the tags on request, adding any that do not yet exist in the tags table
        for tag in deduped_tag_list.clone() {
            if !existing_tags.contains(&tag) {
                tags_to_create.push(tag);
            }
        }

        // create the article so we can reference the created article tags
        let slug = slugify(&title);

        let created_article = self
            .articles_repository
            .create_article(user_id, title, slug, description, body)
            .await?;

        // if we detect new tags, create them
        if !tags_to_create.is_empty() {
            self.tags_repository.create_tags(tags_to_create).await?;
        }

        // re-query the tags table to get all the existing tags with their associated IDs
        // while mapping them into a tuple of tag IDs and article ID so we can create
        // the related article tags for the article
        let article_tags_to_create = self
            .tags_repository
            .get_tags(deduped_tag_list.clone())
            .await?
            .into_iter()
            .map(|tag| (tag.id, created_article.id))
            .collect_vec();

        // finally, create the article tags
        self.tags_repository
            .create_article_tags(article_tags_to_create)
            .await?;

        Ok(created_article.into_dto(deduped_tag_list))
    }

    async fn get_articles(
        &self,
        user_id: Option<i64>,
        tag: Option<String>,
        author: Option<String>,
        favorited: Option<String>,
        limit: i64,
        offset: i64,
    ) -> ConduitResult<Vec<ArticleDto>> {
        let articles = self
            .articles_repository
            .get_articles(user_id, tag, author, favorited, limit, offset)
            .await?;

        info!("found {} articles", articles.len());

        let mut mapped_articles: Vec<ArticleDto> = Vec::new();

        if !articles.is_empty() {
            let article_ids = articles.iter().map(|article| article.id).collect_vec();

            let associated_article_tags = self
                .tags_repository
                .get_article_tags_article_ids(article_ids)
                .await?;

            for article in articles {
                let article_tags = associated_article_tags
                    .iter()
                    .filter(|article_tag| article_tag.article_id == article.id)
                    .map(|tag| tag.tag.clone())
                    .collect_vec();

                mapped_articles.push(article.into_dto(article_tags));
            }
        }

        Ok(mapped_articles)
    }

    async fn get_article(&self, user_id: Option<i64>, slug: String) -> ConduitResult<ArticleDto> {
        info!("retrieving article {:?}", slug);
        let article = self
            .articles_repository
            .get_article_by_slug(user_id, slug)
            .await?;

        if let Some(existing_article) = article {
            info!(
                "retrieving article tags for article {:?}",
                existing_article.id
            );
            let article_tags = self
                .tags_repository
                .get_article_tags_by_article_id(existing_article.id)
                .await?
                .into_iter()
                .map(|article_tag| article_tag.tag)
                .collect_vec();

            return Ok(existing_article.into_dto(article_tags));
        }

        Err(ConduitError::NotFound(String::from("article not found")))
    }
}
