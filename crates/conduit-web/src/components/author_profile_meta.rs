use yew::prelude::*;
use yew_router::prelude::*;

use crate::{router::ConduitRouter, utilities::formatting::convert_to_friend_date_string};

#[derive(Properties, PartialEq, Clone)]
pub struct AuthorProfileMetaProps {
    pub username: String,
    pub image: String,
    pub created_date: String,
}

#[function_component(AuthorProfileMeta)]
pub fn author_profile_meta(props: &AuthorProfileMetaProps) -> Html {
    let formatted_date = use_state(String::default);

    {
        let props = props.clone();
        let formatted_date = formatted_date.clone();

        use_effect_with_deps(
            move |current_props| {
                let formatted = convert_to_friend_date_string(current_props.created_date.clone());

                if let Ok(as_formatted_date) = formatted {
                    formatted_date.set(as_formatted_date);
                }

                || ()
            },
            props,
        );
    }

    html! {
        <>
            <Link<ConduitRouter> to={ConduitRouter::Profile { username: props.username.clone() }}>
                <img src={props.image.clone()} />
            </Link<ConduitRouter>>
            <div class="info">
                <Link<ConduitRouter> classes="author" to={ConduitRouter::Profile { username: props.username.clone() }}>
                    {props.username.clone()}
                </Link<ConduitRouter>>
                <span class="date">{(*formatted_date).clone()}</span>
            </div>
        </>
    }
}
