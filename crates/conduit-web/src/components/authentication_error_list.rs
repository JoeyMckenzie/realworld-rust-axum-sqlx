use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AuthenticationErrorListProps {
    pub errors: Vec<String>,
}

#[function_component(AuthenticationErrorList)]
pub fn authentication_error_list(props: &AuthenticationErrorListProps) -> Html {
    let mapped_errors = props
        .clone()
        .errors
        .into_iter()
        .map(|error| {
            html! {
                <li>{ error }</li>
            }
        })
        .collect::<Html>();

    html! {
        <ul class="error-messages">
            { mapped_errors }
        </ul>
    }
}
