use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::ConduitRouter;

#[derive(Properties, PartialEq, Clone)]
pub struct ActiveLinkProps {
    pub to: ConduitRouter,
    pub display_as: &'static str,
}

#[function_component(ActiveLink)]
pub fn active_link(props: &ActiveLinkProps) -> Html {
    let route = use_route::<ConduitRouter>().expect("could not load current route");
    let active_classes = use_state_eq(|| "");

    {
        let props = props.clone();
        let active_classes = active_classes.clone();

        use_effect_with_deps(
            move |&current_route| {
                if current_route == props.to {
                    active_classes.set("nav-link active");
                } else {
                    active_classes.set("nav-link");
                }
                || ()
            },
            route,
        );
    }

    html! {
        <Link<ConduitRouter> classes={*active_classes} to={props.to}>{ props.display_as }</Link<ConduitRouter>>
    }
}
