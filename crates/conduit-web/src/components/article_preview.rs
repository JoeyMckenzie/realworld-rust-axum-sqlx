use conduit_domain::articles::models::AuthorDto;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::author_profile_meta::AuthorProfileMeta, router::ConduitRouter};

#[derive(Properties, PartialEq, Clone)]
pub struct ArticlePreviewProps {
    pub author: AuthorDto,
    pub title: String,
    pub slug: String,
    pub created_date: String,
    pub description: String,
    pub favorites: usize,
}

#[function_component(ArticlePreview)]
pub fn article_preview(props: &ArticlePreviewProps) -> Html {
    html! {
        <div class="article-preview">
            <div class="article-meta">
                <AuthorProfileMeta
                    username={props.author.username.clone()}
                    image={props.author.image.as_ref().unwrap_or(&String::default()).to_owned()}
                    created_date={props.created_date.clone()}
                />
                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                    <i class="ion-heart"></i> {props.favorites}
                </button>
            </div>
            <Link<ConduitRouter> classes="preview-link" to={ConduitRouter::Article { slug: props.slug.clone() }}>
                <h1>{props.title.clone()}</h1>
                <p>{props.description.clone()}</p>
                <>
                    { "Read more..." }
                </>
            </Link<ConduitRouter>>
        </div>
    }
}
