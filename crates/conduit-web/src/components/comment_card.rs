use yew::prelude::*;

use crate::utilities::formatting::convert_to_friend_date_string;

#[derive(Properties, PartialEq, Clone)]
pub struct CommentCardProps {
    pub image: String,
    pub username: String,
    pub created_date: String,
}

#[function_component(CommentCard)]
pub fn comment_card(props: &CommentCardProps) -> Html {
    let formatted_date = {
        let props = props.clone();
        convert_to_friend_date_string(props.created_date).unwrap_or_default()
    };

    html! {
        <div class="card">
            <div class="card-block">
                <p class="card-text">{"With supporting text below as a natural lead-in to additional content."}</p>
            </div>
            <div class="card-footer">
                <a href="" class="comment-author">
                    <img src={props.image.clone()} class="comment-author-img" />
                </a>
                {"\u{00a0}"}
                <a href="" class="comment-author">{props.username.clone()}</a>
                <span class="date-posted">{formatted_date}</span>
            </div>
        </div>
    }
}
