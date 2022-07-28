use conduit_domain::articles::models::ArticleDto;
use log::info;
use yew::prelude::*;

use crate::components::article_preview::ArticlePreview;

#[derive(Properties, PartialEq, Clone)]
pub struct ArticlePreviewsProps {
    pub articles: Vec<ArticleDto>,
}

#[function_component(ArticlePreviews)]
pub fn article_previews(props: &ArticlePreviewsProps) -> Html {
    let props = props.clone();

    {
        let props = props.clone();

        use_effect(move || {
            info!("articles {}", props.articles.len());
            || ()
        });
    }

    let mapped_articles = props
        .articles
        .into_iter()
        .map(|article| {
            html! {
                <ArticlePreview
                    author={article.author}
                    title={article.title}
                    slug={article.slug}
                    created_date={article.created_at}
                    description={article.description}
                    favorites={article.favorites_count as usize}
                />
            }
        })
        .collect::<Html>();

    html! {
        <>
            {mapped_articles}
        </>
    }
}
