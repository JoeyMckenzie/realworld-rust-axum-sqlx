use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::ConduitRouter;

#[derive(Properties, PartialEq, Clone)]
pub struct ActiveLinkProps {
    pub to: ConduitRouter,
    pub display_as: String,
}

#[function_component(ActiveLink)]
pub fn active_link(props: &ActiveLinkProps) -> Html {
    let active_classes = use_state_eq(|| "");

    {
        let route = use_route::<ConduitRouter>().expect("failed to load current route");
        let props = props.clone();
        let active_classes = active_classes.clone();

        use_effect(move || {
            if route == props.to {
                active_classes.set("nav-link active");
            } else {
                active_classes.set("nav-link");
            }
            || ()
        });
    }

    html! {
        <Link<ConduitRouter> classes={*active_classes} to={props.to.clone()}>{ props.display_as.clone() }</Link<ConduitRouter>>
    }
}
