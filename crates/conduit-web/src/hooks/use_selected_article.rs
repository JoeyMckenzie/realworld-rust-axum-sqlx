use conduit_domain::articles::models::ArticleDto;
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::services::article_service::get_article;

#[derive(Debug)]
pub struct UseSelectedArticleHook {
    pub article: ArticleDto,
}

pub fn use_selected_article(slug: String) -> UseSelectedArticleHook {
    let selected_article = use_state(ArticleDto::default);

    {
        let selected_article = selected_article.clone();

        use_effect_with_deps(
            move |current_slug| {
                let current_slug = current_slug.clone();

                spawn_local(async move {
                    let response = get_article(current_slug.clone()).await;

                    if let Ok(article) = response {
                        info!("article {} successfully loaded", current_slug);
                        selected_article.set(article.article);
                    } else {
                        error!("error while loading article {}", current_slug);
                    }
                });

                || ()
            },
            slug,
        )
    }

    UseSelectedArticleHook {
        article: (*selected_article).clone(),
    }
}
