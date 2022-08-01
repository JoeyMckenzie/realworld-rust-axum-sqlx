use yew::prelude::*;

#[function_component(FeedToggle)]
pub fn feed_toggle() -> Html {
    html! {
        <div class="feed-toggle">
            <ul class="nav nav-pills outline-active">
                <li class="nav-item">
                    <a class="nav-link disabled" href="">{ "Your Feed" }</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link active" href="">{ "Global Feed" }</a>
                </li>
            </ul>
        </div>
    }
}
