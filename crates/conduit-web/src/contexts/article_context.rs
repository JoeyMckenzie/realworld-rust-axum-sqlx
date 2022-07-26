use std::rc::Rc;

use conduit_domain::articles::models::ArticleDto;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ArticleState {
    pub current_article: Option<ArticleDto>,
}

impl ArticleState {
    pub fn new() -> Self {
        Self { current_article: None }
    }
}

impl Reducible for ArticleState {
    type Action = ArticleDto;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        ArticleState {
            current_article: Some(action),
        }
        .into()
    }
}

type ArticleContext = UseReducerHandle<ArticleState>;

#[derive(Properties, Debug, PartialEq)]
pub struct ArticleProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ArticleProvider)]
pub fn article_provider(props: &ArticleProviderProps) -> Html {
    let article_state = use_reducer(ArticleState::new);

    html! {
        <ContextProvider<ArticleContext> context={article_state}>
            {props.children.clone()}
        </ContextProvider<ArticleContext>>
    }
}

pub fn use_article_context() -> ArticleContext {
    use_context::<ArticleContext>().expect("article context was not loaded")
}
