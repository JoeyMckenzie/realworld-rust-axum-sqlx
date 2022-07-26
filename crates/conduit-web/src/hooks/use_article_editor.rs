use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlInputElement, HtmlTextAreaElement, InputEvent};
use yew::prelude::*;

use crate::services::article_service::create_article;

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
    let title = use_state(String::default);
    let description = use_state(String::default);
    let body = use_state(String::default);
    let tags = use_state(String::new);

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

                if article_response.is_ok() {
                    info!("article {} successfully created", (*title).clone());
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