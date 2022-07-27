use yew::prelude::*;

use crate::{
    components::{author_profile_meta::AuthorProfileMeta, comment_card::CommentCard},
    contexts::authentication_context::use_authentication_context,
    hooks::use_selected_article::{use_selected_article, UseSelectedArticleHook},
};

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleProps {
    pub slug: String,
}

#[function_component(Article)]
pub fn article(props: &ArticleProps) -> Html {
    let UseSelectedArticleHook { article, comments } = use_selected_article(props.slug.clone());
    let authentication_context = use_authentication_context();

    let maybe_follow_and_post_buttons = {
        let authentication_context = authentication_context.clone();
        let article = article.clone();

        move || -> Html {
            let follow_button_text = if article.author.following { "Unfollow" } else { "Follow" };
            let favorite_button_text = if article.favorited { "Unfavorite" } else { "Favorite" };

            if authentication_context.is_authenticated() {
                html! {
                    <>
                        <AuthorProfileMeta
                            username={article.author.username.clone()}
                            image={article.author.image.as_ref().unwrap_or(&String::default()).to_owned()}
                            created_date={article.created_at.clone()}
                        />
                        <button class="btn btn-sm btn-outline-secondary">
                            <i class="ion-plus-round"></i>
                            {format!("\u{00a0}{} {}", follow_button_text, article.author.username)}
                        </button>
                        {"\u{00a0}\u{00a0}"}
                        <button class="btn btn-sm btn-outline-primary">
                            <i class="ion-heart"></i>
                            {format!("\u{00a0}{} Post", favorite_button_text)} <span class="counter">{format!(" \u{0028}{}\u{0029}", article.favorites_count)}</span>
                        </button>
                    </>
                }
            } else {
                html! {
                    <AuthorProfileMeta
                        username={article.author.username.clone()}
                        image={article.author.image.as_ref().unwrap_or(&String::default()).to_owned()}
                        created_date={article.created_at.clone()}
                    />
                }
            }
        }
    };

    let maybe_comment_box = {
        let authentication_context = authentication_context.clone();

        move || -> Html {
            if authentication_context.is_authenticated() {
                html! {
                    <form class="card comment-form">
                        <div class="card-block">
                            <textarea class="form-control" placeholder="Write a comment..." rows="3"></textarea>
                        </div>
                        <div class="card-footer">
                            <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
                            <button class="btn btn-sm btn-primary">
                                {"Post Comment"}
                            </button>
                        </div>
                    </form>
                }
            } else {
                html! {}
            }
        }
    };

    let maybe_edit_comment_options = {
        move || -> Html {
            if authentication_context.is_authenticated() {
                html! {
                    <span class="mod-options">
                        <i class="ion-edit"></i>
                        <i class="ion-trash-a"></i>
                    </span>
                }
            } else {
                html! {}
            }
        }
    };

    let user_comments = move || -> Html {
        let mapped_comments = comments
            .into_iter()
            .map(|comment| {
                html! {
                    <CommentCard
                        username={comment.author.username.clone()}
                        image={comment.author.image.as_ref().unwrap_or(&String::default()).to_owned()}
                        created_date={comment.created_at.clone()}
                    />
                }
            })
            .collect::<Vec<Html>>();

        html! {
            <>
                {mapped_comments}
            </>
        }
    };

    html! {
        <div class="article-page">
            <div class="banner">
                <div class="container">
                    <h1>{article.title.clone()}</h1>

                    <div class="article-meta">
                        {maybe_follow_and_post_buttons()}
                    </div>
                </div>
            </div>

            <div class="container page">
                <div class="row article-content">
                    <div class="col-md-12">
                        <p>
                            {article.description}
                        </p>
                        <h2 id="introducing-ionic">{article.title}</h2>
                        <p>{article.body}</p>
                    </div>
                </div>

                <hr />

                <div class="article-actions">
                    <div class="article-meta">
                        {maybe_follow_and_post_buttons()}
                    </div>
                </div>

                <div class="row">
                    <div class="col-xs-12 col-md-8 offset-md-2">
                        {maybe_comment_box()}

                        {user_comments()}

                        <div class="card">
                            <div class="card-block">
                                <p class="card-text">{"With supporting text below as a natural lead-in to additional content."}</p>
                            </div>
                            <div class="card-footer">
                                <a href="" class="comment-author">
                                    <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
                                </a>
                                {"\u{00a0}"}
                                <a href="" class="comment-author">{"Jacob Schmidt"}</a>
                                <span class="date-posted">{"Dec 29th"}</span>
                                {maybe_edit_comment_options()}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
