use std::rc::Rc;

use conduit_domain::articles::models::ArticleDto;
use lazy_static::lazy_static;
use yew::prelude::*;

lazy_static! {
    static ref DEFAULT_LIMIT: usize = 20;
    static ref DEFAULT_OFFSET: usize = 0;
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArticleState {
    pub articles: Vec<ArticleDto>,
    pub tags: Vec<String>,
}

pub enum ArticleActions {
    SetArticles(Vec<ArticleDto>),
    SetTags(Vec<String>),
}

impl ArticleState {
    pub fn new() -> Self {
        Self {
            articles: Vec::<ArticleDto>::default(),
            tags: Vec::<String>::default(),
        }
    }
}

impl Reducible for ArticleState {
    type Action = ArticleActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ArticleActions::SetArticles(articles) => ArticleState {
                articles,
                tags: self.tags.clone(),
            }
            .into(),
            ArticleActions::SetTags(tags) => ArticleState {
                articles: self.articles.clone(),
                tags,
            }
            .into(),
        }
    }
}

pub type ArticleContext = UseReducerHandle<ArticleState>;

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
