use conduit_domain::tags::responses::TagsResponse;
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utilities::http::get;

#[function_component(TagList)]
pub fn tag_list() -> Html {
    let tags = use_state(Vec::<String>::new);

    {
        let tags = tags.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let tags_response = get::<TagsResponse>("/api/tags").await;

                    if let Ok(tags_list) = tags_response {
                        info!("tags successfully retrieved, found {} tags", tags_list.tags.len());
                        tags.set(tags_list.tags);
                    } else {
                        error!("error while retrieving tags");
                    }
                });
                || ()
            },
            (),
        );
    }

    let tags_listing = move || -> Html {
        tags.iter()
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
