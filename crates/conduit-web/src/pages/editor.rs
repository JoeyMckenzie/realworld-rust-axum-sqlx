use yew::prelude::*;

use crate::hooks::use_article_editor::{use_article_editor, UseArticleEditorHook};

#[function_component(Editor)]
pub fn editor() -> Html {
    let UseArticleEditorHook {
        title,
        title_oninput,
        description,
        description_oninput,
        body,
        body_oninput,
        tags,
        tags_oninput,
        onsubmit,
    } = use_article_editor();

    html! {
        <div class="editor-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-10 offset-md-1 col-xs-12">
                        <form onsubmit={onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        type="text"
                                        class="form-control form-control-lg"
                                        placeholder="Article Title"
                                        value={title}
                                        oninput={title_oninput}
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        type="text"
                                        class="form-control"
                                        placeholder="What's this article about?"
                                        value={description}
                                        oninput={description_oninput}
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control"
                                        rows="8"
                                        placeholder="Write your article (in markdown)"
                                        value={body}
                                        oninput={body_oninput}>
                                    </textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        type="text"
                                        class="form-control"
                                        placeholder="Enter tags"
                                        value={tags}
                                        oninput={tags_oninput}
                                    />
                                    <div class="tag-list"></div>
                                </fieldset>
                                <button class="btn btn-lg pull-xs-right btn-primary" type="submit">
                                    { "Publish Article" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
