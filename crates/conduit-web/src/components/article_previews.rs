use conduit_domain::articles::models::ArticleDto;
use yew::prelude::*;

use crate::components::article_preview::ArticlePreview;

#[derive(Properties, PartialEq, Clone)]
pub struct ArticlePreviewsProps {
    pub articles: Vec<ArticleDto>,
}

#[function_component(ArticlePreviews)]
pub fn article_previews(props: &ArticlePreviewsProps) -> Html {
    let mapped_articles = props
        .clone()
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
