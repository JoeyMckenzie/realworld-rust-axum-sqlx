use conduit_domain::tags::responses::TagsResponse;
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    contexts::articles_context::{use_article_context, ArticleActions},
    utilities::http::get,
};

#[function_component(TagList)]
pub fn tag_list() -> Html {
    let context = use_article_context();

    use_effect_with_deps(
        move |current_context| {
            let current_context = current_context.clone();

            spawn_local(async move {
                let tags_response = get::<TagsResponse>("/tags").await;

                if let Ok(tags_list) = tags_response {
                    info!("tags successfully retrieved, found {} tags", tags_list.tags.len());
                    current_context.dispatch(ArticleActions::SetTags(tags_list.tags));
                } else {
                    error!("error while retrieving tags");
                }
            });
            || ()
        },
        context.clone(),
    );

    let tags_listing = move || -> Html {
        context
            .tags
            .iter()
            .map(|tag| {
                html! {
                    <a href="" class="tag-pill tag-default">{ tag }</a>
                }
            })
            .collect::<Html>()
    };

    html! {
        <div class="col-md-3">
            <div class="sidebar">
                <p>{ "Popular Tags" }</p>
                <div class="tag-list">
                    {tags_listing()}
                </div>
            </div>
        </div>
    }
}
