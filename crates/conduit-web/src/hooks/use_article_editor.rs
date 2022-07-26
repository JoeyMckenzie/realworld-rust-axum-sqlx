use log::{error, info, warn};
use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlInputElement, HtmlTextAreaElement, InputEvent};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    contexts::authentication_context::use_authentication_context, router::ConduitRouter,
    services::article_service::create_article,
};

#[derive(Debug)]
pub struct UseArticleEditorHook {
    pub title: String,
    pub title_oninput: Callback<InputEvent>,
    pub description: String,
    pub description_oninput: Callback<InputEvent>,
    pub body: String,
    pub body_oninput: Callback<InputEvent>,
    pub tags: String,
    pub tags_oninput: Callback<InputEvent>,
    pub onsubmit: Callback<FocusEvent>,
}

pub fn use_article_editor() -> UseArticleEditorHook {
    let history = use_history().expect("history failed to load");
    let title = use_state(String::default);
    let description = use_state(String::default);
    let body = use_state(String::default);
    let tags = use_state(String::new);

    {
        let authentication_context = use_authentication_context();
        let history = history.clone();

        use_effect_with_deps(
            move |(context, history_location)| {
                if !context.is_authenticated() {
                    warn!("user is not authenticated, redirecting to home");
                    history_location.push(ConduitRouter::Home);
                }
                || ()
            },
            (authentication_context, history),
        );
    }

    let title_oninput = {
        let title = title.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let description_oninput = {
        let description = description.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    let body_oninput = {
        let body = body.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            body.set(input.value());
        })
    };

    let tags_oninput = {
        let tags = tags.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            tags.set(input.value());
        })
    };

    let onsubmit = {
        let title = title.clone();
        let description = description.clone();
        let body = body.clone();
        let tags = tags.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let history = history.clone();
            let title = title.clone();
            let description = description.clone();
            let body = body.clone();
            let tags = tags.clone();

            // split the tags input, removing any non-alpha characters
            let split_tags = tags
                .split(' ')
                .map(|c| c.to_owned())
                .filter(|tag| !tag.is_empty())
                .collect::<Vec<String>>();

            spawn_local(async move {
                let article_response =
                    create_article((*title).clone(), (*description).clone(), (*body).clone(), split_tags).await;

                if let Ok(created_article) = article_response {
                    info!("article {} successfully created", (*title).clone());
                    history.push(ConduitRouter::Article {
                        slug: created_article.article.slug,
                    });
                } else {
                    error!("error while creating article {}", (*title).clone());
                }
            });
        })
    };

    UseArticleEditorHook {
        title: (*title).clone(),
        title_oninput,
        description: (*description).clone(),
        description_oninput,
        body: (*body).clone(),
        body_oninput,
        tags: (*tags).clone(),
        tags_oninput,
        onsubmit,
    }
}
