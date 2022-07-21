use yew::prelude::*;

use crate::components::authentication_form::AuthenticationForm;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <AuthenticationForm include_username={false} />
    }
}
