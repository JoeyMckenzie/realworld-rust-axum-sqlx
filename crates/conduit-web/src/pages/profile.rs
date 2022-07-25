use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{contexts::authentication_context::use_authentication_context, services::profile_service::get_profile};

#[derive(Properties, PartialEq, Clone)]
pub struct ProfileProps {
    pub username: String,
}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let authentication_context = use_authentication_context();
    let image = use_state(String::default);
    let bio = use_state(String::default);
    let username = use_state(String::default);
    let following = use_state(|| false);

    {
        let props = props.clone();
        let authentication_context = authentication_context.clone();
        let image = image.clone();
        let bio = bio.clone();
        let username = username.clone();
        let following = following.clone();

        use_effect_with_deps(
            move |_| {
                // in the case the current user is navigating to their profile, pull there info from state
                if authentication_context
                    .username
                    .as_ref()
                    .unwrap_or(&String::default())
                    .eq(&*props.username)
                {
                    bio.set(authentication_context.bio.as_ref().unwrap().clone());
                    username.set(authentication_context.username.as_ref().unwrap().clone());
                    image.set(authentication_context.image.as_ref().unwrap().clone());
                    following.set(false);
                } else {
                    spawn_local(async move {
                        let current_profile_response = get_profile(props.username).await;

                        if let Ok(current_profile) = current_profile_response {
                            bio.set(current_profile.bio);
                            username.set(current_profile.username);
                            following.set(current_profile.following);
                            image.set(current_profile.image);
                        }
                    });
                }
                || ()
            },
            (),
        );
    }

    let maybe_following_button = {
        let username = username.clone();

        move || -> Html {
            // if the current profile is the authenticated user, don't display the follow/unfollow button
            if authentication_context
                .username
                .as_ref()
                .unwrap_or(&String::default())
                .eq(&*username)
            {
                return html! {};
            }

            if *following {
                html! {
                    <button class="btn btn-sm btn-outline-secondary action-btn">
                        <i class="ion-plus-round"></i>
                        { &format!("\u{00a0}Follow {}", (*username).clone()) }
                    </button>
                }
            } else {
                html! {
                    <button class="btn btn-sm btn-outline-secondary action-btn">
                        <i class="ion-plus-round"></i>
                        { &format!("\u{00a0}Unfollow {}", (*username).clone()) }
                    </button>
                }
            }
        }
    };

    html! {
        <div class="profile-page">
            <div class="user-info">
                <div class="container">
                    <div class="row">
                        <div class="col-xs-12 col-md-10 offset-md-1">
                            <img src={(*image).clone()} class="user-img"/>
                            <h4>{ (*username).clone() }</h4>
                            <p>
                                { (*bio).clone() }
                            </p>
                            {maybe_following_button()}
                        </div>

                    </div>
                </div>
            </div>

            <div class="container">
                <div class="row">
                    <div class="col-xs-12 col-md-10 offset-md-1">
                        <div class="articles-toggle">
                            <ul class="nav nav-pills outline-active">
                                <li class="nav-item">
                                    <a class="nav-link active" href="">{"My Articles"}</a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link" href="">{"Favorited Articles"}</a>
                                </li>
                            </ul>
                        </div>

                        <div class="article-preview">
                            <div class="article-meta">
                                <a href=""><img src="http://i.imgur.com/Qr71crq.jpg"/></a>
                                <div class="info">
                                    <a href="" class="author">{"Eric Simons"}</a>
                                    <span class="date">{"January 20th"}</span>
                                </div>
                                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                                    <i class="ion-heart"></i> { 29_usize }
                                </button>
                            </div>
                            <a href="" class="preview-link">
                                <h1>{"How to build webapps that scale"}</h1>
                                <p>{"This is the description for the post."}</p>
                                <span>{"Read more..."}</span>
                            </a>
                        </div>

                        <div class="article-preview">
                            <div class="article-meta">
                                <a href=""><img src="http://i.imgur.com/N4VcUeJ.jpg"/></a>
                                <div class="info">
                                    <a href="" class="author">{"Albert Pai"}</a>
                                    <span class="date">{"January 20th"}</span>
                                </div>
                                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                                    <i class="ion-heart"></i> { 32_usize }
                                </button>
                            </div>
                            <a href="" class="preview-link">
                                <h1>{"The song you won't ever stop singing. No matter how hard you try."}</h1>
                                <p>{"This is the description for the post."}</p>
                                <span>{"Read more..."}</span>
                                <ul class="tag-list">
                                    <li class="tag-default tag-pill tag-outline">{"Music"}</li>
                                    <li class="tag-default tag-pill tag-outline">{"Song"}</li>
                                </ul>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
