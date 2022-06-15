use std::collections::HashSet;

use async_trait::async_trait;
use itertools::Itertools;
use slug::slugify;
use tracing::info;

use conduit_core::articles::repository::DynArticlesRepository;
use conduit_core::articles::service::ArticlesService;
use conduit_core::errors::ConduitResult;
use conduit_core::tags::repository::DynTagsRepository;
use conduit_core::users::repository::DynUsersRepository;
use conduit_domain::articles::models::ArticleDto;
use conduit_domain::articles::requests::GetArticlesServiceRequest;

pub struct ConduitArticlesService {
    articles_repository: DynArticlesRepository,
    tags_repository: DynTagsRepository,
    users_repository: DynUsersRepository,
}

impl ConduitArticlesService {
    pub fn new(
        articles_repository: DynArticlesRepository,
        tags_repository: DynTagsRepository,
        users_repository: DynUsersRepository,
    ) -> Self {
        Self {
            articles_repository,
            tags_repository,
            users_repository,
        }
    }
}

#[async_trait]
impl ArticlesService for ConduitArticlesService {
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

        let authors = articles
            .into_iter()
            .map(|entity| entity.id)
            .collect::<HashSet<i64>>();

        todo!("get associated authors and tags")
    }
}
