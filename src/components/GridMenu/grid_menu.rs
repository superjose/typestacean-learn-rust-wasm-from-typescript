use yew::prelude::*;
use yew_hooks::prelude::use_location;

use crate::components::GridItem::grid_item::GridItem;

#[derive(Clone, PartialEq, Properties)]
pub struct Item {
    pub text: String,
    pub href: String,
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
                    <a href={item.href}>
                        <GridItem key={item.text} active={item.href == &location.pathname}>
                            {item.text}
                        </GridItem>
                    </a>
                }
            }).collect::<Html>()}
    </div>
    }
}
