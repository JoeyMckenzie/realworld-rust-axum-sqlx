use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::ConduitRouter;

#[derive(Properties, PartialEq, Clone)]
pub struct AuthorProfileMetaProps {
    pub username: String,
    pub image: String,
    pub created_date: String,
}

#[function_component(AuthorProfileMeta)]
pub fn author_profile_meta(props: &AuthorProfileMetaProps) -> Html {
    html! {
        <>
            <Link<ConduitRouter> to={ConduitRouter::Profile { username: props.username.clone() }}>
                <img src={props.image.clone()} />
            </Link<ConduitRouter>>
            <div class="info">
                <Link<ConduitRouter> classes="author" to={ConduitRouter::Profile { username: props.username.clone() }}>
                    {props.username.clone()}
                </Link<ConduitRouter>>
                <span class="date">{props.created_date.clone()}</span>
            </div>
        </>
    }
}
