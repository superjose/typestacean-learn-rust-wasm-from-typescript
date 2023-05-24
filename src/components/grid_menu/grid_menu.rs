use yew::prelude::*;
use yew_hooks::prelude::use_location;
use yew_router::{prelude::Link, Routable};

use crate::{components::grid_item::grid_item::GridItem, utils::routes::Route};

impl Route {
    fn is_active(&self, pathname: String) -> bool {
        self.to_path() == pathname
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Item {
    pub text: String,
    pub route: Route,
}

#[derive(Properties, Clone, PartialEq)]
pub struct GridMenuProps {
    pub items: Vec<Item>,
}

#[function_component(GridMenu)]
pub fn grid_menu(props: &GridMenuProps) -> Html {
    let location = use_location();
    html! {
        <div className="grid grid-cols-1 md:grid-cols-3">
            {props.items.iter().map(|item| {
                html! {
                    <Link<Route> to={item.route.clone()}>
                        <GridItem key={item.text.clone()} active={item.route.is_active(location.pathname.clone())}>
                            {&item.text}
                        </GridItem>
                    </Link<Route>>
                }
            }).collect::<Html>()}
    </div>
    }
}
