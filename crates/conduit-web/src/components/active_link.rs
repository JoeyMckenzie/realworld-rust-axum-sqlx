use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::ConduitRouter;

#[derive(Properties, PartialEq, Clone)]
pub struct ActiveLinkProps {
    #[prop_or_default]
    pub children: Children,
    pub to: ConduitRouter,
}

#[function_component(ActiveLink)]
pub fn active_link(props: &ActiveLinkProps) -> Html {
    let active_classes = use_state_eq(|| "");
    let route = use_route::<ConduitRouter>().expect("failed to load current route");

    use_effect_with_deps(
        move |(current_route, current_props, current_active_classes)| {
            if *current_route == current_props.to {
                current_active_classes.set("nav-link active");
            } else {
                current_active_classes.set("nav-link");
            }
            || ()
        },
        (route, props.clone(), active_classes.clone()),
    );

    html! {
        <Link<ConduitRouter> classes={*active_classes} to={props.to.clone()}>
            { props.children.clone() }
        </Link<ConduitRouter>>
    }
}
