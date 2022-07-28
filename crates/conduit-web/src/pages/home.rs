use yew::prelude::*;

use crate::{
    components::{article_previews::ArticlePreviews, feed_toggle::FeedToggle, tag_list::TagList},
    hooks::use_selected_articles::{use_selected_articles, UseSelectedArticlesHook},
};

#[function_component(Home)]
pub fn home() -> Html {
    let limit = use_state(|| 20_usize);
    let offset = use_state(|| 0_usize);
    let author = use_state(String::default);
    let tag = use_state(String::default);
    let favorited = use_state(String::default);

    let UseSelectedArticlesHook { articles } =
        use_selected_articles(*limit, *offset, (*author).clone(), (*tag).clone(), (*favorited).clone());

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
                        <ArticlePreviews articles={articles} />
                    </div>
                    <TagList />
                </div>
            </div>
        </div>
    }
}
