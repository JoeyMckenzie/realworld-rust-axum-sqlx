use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    components::{article_previews::ArticlePreviews, feed_toggle::FeedToggle, tag_list::TagList},
    contexts::articles_context::{use_article_context, ArticleActions},
    services::article_service::get_articles,
};

#[function_component(Home)]
pub fn home() -> Html {
    let limit = use_state(|| 20_usize);
    let offset = use_state(|| 0_usize);
    let author = use_state(String::default);
    let tag = use_state(String::default);
    let favorited = use_state(String::default);

    let context = use_article_context();

    use_effect_with_deps(
        move |(
            current_limit,
            current_offset,
            current_author,
            current_tag,
            current_favorited,
            current_articles_context,
        )| {
            let current_limit = (*current_limit).clone();
            let current_offset = (*current_offset).clone();
            let current_author = current_author.clone();
            let current_tag = current_tag.clone();
            let current_favorited = current_favorited.clone();
            let current_articles_context = current_articles_context.clone();

            spawn_local(async move {
                info!(
                    "retrieving articles: limit={}, offset={}, author=\"{}\", tag=\"{}\", favorited=\"{}\"",
                    *current_limit, *current_offset, *current_author, *current_tag, *current_favorited
                );

                let articles_response = get_articles(
                    *current_limit,
                    *current_offset,
                    (*current_author).clone(),
                    (*current_tag).clone(),
                    (*current_favorited).clone(),
                )
                .await;

                if let Ok(articles_from_response) = articles_response {
                    info!(
                        "successfully retrieved {} articles",
                        articles_from_response.articles_count
                    );
                    current_articles_context.dispatch(ArticleActions::SetArticles(articles_from_response.articles));
                } else {
                    error!("error while retrieving articles");
                }
            });

            || ()
        },
        (limit, offset, author, tag, favorited, context.clone()),
    );

    html! {
        <div class="home-page">
            <div class="banner">
                <div class="container">
                    <h1 class="logo-font">{ "conduit" }</h1>
                    <p>{ "A place to share your knowledge." }</p>
                </div>
            </div>

            <div class="container page">
                <div class="row">
                    <div class="col-md-9">
                        <FeedToggle />
                        <ArticlePreviews articles={context.articles.clone()} />
                    </div>
                    <TagList />
                </div>
            </div>
        </div>
    }
}
