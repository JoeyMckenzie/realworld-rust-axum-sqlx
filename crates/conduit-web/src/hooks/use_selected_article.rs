use conduit_domain::{articles::models::ArticleDto, comments::CommentDto};
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::services::article_service::{get_article, get_article_comments};

#[derive(Debug)]
pub struct UseSelectedArticleHook {
    pub article: ArticleDto,
    pub comments: Vec<CommentDto>,
}

pub fn use_selected_article(slug: String) -> UseSelectedArticleHook {
    let selected_article = use_state(ArticleDto::default);
    let comments = use_state(Vec::<CommentDto>::default);

    {
        use_effect_with_deps(
            move |(current_slug, current_selected_article, current_comments)| {
                let current_slug = current_slug.clone();
                let current_comments = current_comments.clone();
                let current_selected_article = current_selected_article.clone();

                spawn_local(async move {
                    let response = get_article(current_slug.clone()).await;

                    if let Ok(article) = response {
                        info!("article {} successfully loaded, loading comments", current_slug);

                        current_selected_article.set(article.article);
                        let comments_response = get_article_comments(current_slug.clone()).await;

                        if let Ok(loaded_comments) = comments_response {
                            info!("comments loaded for article {}", current_slug);
                            current_comments.set(loaded_comments.comments);
                        } else {
                            error!("error while loading comments for article {}", current_slug);
                        }
                    } else {
                        error!("error while loading article {}", current_slug);
                    }
                });

                || ()
            },
            (slug, selected_article.clone(), comments.clone()),
        )
    }

    UseSelectedArticleHook {
        article: (*selected_article).clone(),
        comments: (*comments).clone(),
    }
}
