use conduit_domain::articles::models::ArticleDto;
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::services::article_service::get_articles;

pub struct UseSelectedArticlesHook {
    pub articles: Vec<ArticleDto>,
}

pub fn use_selected_articles(
    limit: usize,
    offset: usize,
    author: String,
    tag: String,
    favorited: String,
) -> UseSelectedArticlesHook {
    let articles = use_state(Vec::<ArticleDto>::new);

    use_effect_with_deps(
        move |(current_limit, current_offset, current_author, current_tag, current_favorited, current_articles)| {
            let current_articles = current_articles.clone();
            let current_limit = *current_limit;
            let current_offset = *current_offset;
            let current_author = current_author.clone();
            let current_tag = current_tag.clone();
            let current_favorited = current_favorited.clone();

            spawn_local(async move {
                info!(
                    "retrieving articles: limit={}, offset={}, author=\"{}\", tag=\"{}\", favorited=\"{}\"",
                    current_limit, current_offset, current_author, current_tag, current_favorited
                );

                let articles_response = get_articles(
                    current_limit,
                    current_offset,
                    current_author,
                    current_tag,
                    current_favorited,
                )
                .await;

                if let Ok(articles_from_response) = articles_response {
                    info!("successfully retrieved articles");
                    current_articles.set(articles_from_response.articles);
                } else {
                    error!("error while retrieving articles");
                }
            });

            || ()
        },
        (limit, offset, author, tag, favorited, articles.clone()),
    );

    UseSelectedArticlesHook {
        articles: (*articles).clone(),
    }
}
