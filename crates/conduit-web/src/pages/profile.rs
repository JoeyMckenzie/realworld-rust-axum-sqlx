use yew::prelude::*;
use yew_router::prelude::*;

use crate::{contexts::authentication_context::use_authentication_context, router::ConduitRouter};

#[derive(Properties, PartialEq, Clone)]
pub struct ProfileProps {
    pub username: String,
}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let image = "test".to_owned();
    let bio = "test".to_owned();
    let username = "test".to_owned();

    use_effect(move || || ());

    html! {
        <div class="profile-page">
            <div class="user-info">
                <div class="container">
                    <div class="row">
                        <div class="col-xs-12 col-md-10 offset-md-1">
                            <img src={image.clone()} class="user-img"/>
                            <h4>{ username.clone() }</h4>
                            <p>
                                { bio.clone() }
                            </p>
                            <button class="btn btn-sm btn-outline-secondary action-btn">
                                <i class="ion-plus-round"></i>
                                { &format!("\u{00a0}Follow {}", username.clone()) }
                            </button>
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
