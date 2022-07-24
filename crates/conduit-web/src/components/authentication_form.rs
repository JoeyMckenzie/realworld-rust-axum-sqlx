use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::authentication_error_list::AuthenticationErrorList;
use crate::contexts::authentication_context::use_authentication_context;
use crate::hooks::use_authentication::use_authentication;
use crate::router::ConduitRouter;

#[derive(Properties, PartialEq, Clone)]
pub struct AuthenticationFormProps {
    pub include_username: bool,
}

#[function_component(AuthenticationForm)]
pub fn authentication_form(props: &AuthenticationFormProps) -> Html {
    let authentication_context = use_authentication_context();
    let authentication_form_state = use_authentication(props.include_username);
    let history = use_history().expect("history was not loaded");

    use_effect(move || {
        // for the scallywags that try to manually navigate around, kick them out if they're already authenticated
        if authentication_context.is_authenticated() {
            info!("user is already authenticated, navigating home");
            history.push(ConduitRouter::Home);
        }
        || ()
    });

    let onsubmit = authentication_form_state.onsubmit;

    let maybe_display_username = move || -> Html {
        if props.include_username {
            html! {
                <fieldset class="form-group">
                    <input
                        id="authentication-form-username"
                        class="form-control form-control-lg"
                        type="text"
                        placeholder="Your Name"
                        value={authentication_form_state.username}
                        oninput={authentication_form_state.username_oninput}
                    />
                </fieldset>
            }
        } else {
            html! {}
        }
    };

    let account_redirect_link = move || -> Html {
        if props.include_username {
            html! {
                <Link<ConduitRouter> to={ConduitRouter::Login}>{ "Have an account?" }</Link<ConduitRouter>>
            }
        } else {
            html! {
                <Link<ConduitRouter> to={ConduitRouter::Register}>{ "Need an account?" }</Link<ConduitRouter>>
            }
        }
    };

    let submit_button = move || -> Html {
        html! {
            <button
                id="authentication-form-submit-button"
                class="btn btn-lg btn-primary pull-xs-right"
            >
                {
                    if props.include_username {
                        "Sign up"
                    } else {
                        "Sign in"
                    }
                }
            </button>
        }
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign in" }</h1>
                        <p class="text-xs-center">
                            {account_redirect_link()}
                        </p>

                        <AuthenticationErrorList errors={authentication_form_state.errors} />

                        <form {onsubmit}>
                            {maybe_display_username()}
                            <fieldset class="form-group">
                                <input
                                    id="authentication-form-email"
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Email"
                                    value={authentication_form_state.email}
                                    oninput={authentication_form_state.email_oninput}
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    id="authentication-form-password"
                                    class="form-control form-control-lg"
                                    type="password"
                                    placeholder="Password"
                                    value={authentication_form_state.password}
                                    oninput={authentication_form_state.password_oninput}
                                />
                            </fieldset>
                            {submit_button()}
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
