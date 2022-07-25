use yew::prelude::*;

use crate::hooks::use_settings::{use_settings, UseSettingsHook};

#[function_component(Settings)]
pub fn settings() -> Html {
    let UseSettingsHook {
        image,
        image_oninput,
        username,
        username_oninput,
        bio,
        bio_oninput,
        email,
        email_oninput,
        password,
        password_oninput,
        onsubmit,
    } = use_settings();

    html! {
        <div class="settings-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Your Settings" }</h1>
                        <form onsubmit={onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="URL of profile picture"
                                        value={image}
                                        oninput={image_oninput}
                                        />
                                        </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Your Name"
                                        value={username}
                                        oninput={username_oninput}
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control form-control-lg"
                                        rows="8"
                                        placeholder="Short bio about you"
                                        value={bio}
                                        oninput={bio_oninput}
                                    ></textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Email"
                                        value={email}
                                        oninput={email_oninput}
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        value={password}
                                        oninput={password_oninput}
                                    />
                                </fieldset>
                                <button class="btn btn-lg btn-primary pull-xs-right">
                                    { "Update Settings" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
