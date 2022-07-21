use yew::prelude::*;

use crate::components::authentication_form::AuthenticationForm;

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <AuthenticationForm include_username={true} />
    }
}
