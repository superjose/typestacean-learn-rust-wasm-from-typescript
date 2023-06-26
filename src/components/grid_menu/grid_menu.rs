use yew::prelude::*;
use yew_hooks::prelude::use_location;
use yew_router::{prelude::Link, Routable};

use crate::components::grid_item::grid_item::GridItem;

#[derive(Clone, PartialEq, Properties)]
pub struct Item<R>
where
    R: Routable,
{
    pub text: String,
    pub route: R,
}

impl<R> Item<R>
where
    R: Routable,
{
    fn is_active(&self, path: String) -> bool {
        self.route.to_path() == path
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct GridMenuProps<R>
where
    R: Routable,
{
    pub items: Vec<Item<R>>,
}

#[function_component(GridMenu)]
pub fn grid_menu<R>(props: &GridMenuProps<R>) -> Html
where
    R: Routable + 'static,
{
    let location = use_location();
    html! {
        <div className="grid grid-cols-1 md:grid-cols-3">
            {props.items.iter().map(|item| {
                html! {
                    <Link<R> to={item.route.clone()} >
                        <GridItem key={item.text.clone()} active={item.is_active(location.pathname.clone())}>
                            {&item.text}
                        </GridItem>
                    </Link<R>>
                }
            }).collect::<Html>()}
    </div>
    }
}
