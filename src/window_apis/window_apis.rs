use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::grid_menu::grid_menu::{GridMenu, Item};
use crate::components::typography::typography::{Typography, TypographyVariants};

use super::intersection_observer::intersection_observer::IntersectionObserver;

#[derive(Clone, Routable, PartialEq)]
enum WindowApisRoute {
    #[at("/window/intersection_observer")]
    IntersectionObserver,
}

fn switch(routes: WindowApisRoute) -> Html {
    match routes {
        WindowApisRoute::IntersectionObserver => html! { <IntersectionObserver /> },
    }
}

#[function_component(WindowApis)]
pub fn window_apis() -> Html {
    html! {
        <>
        <div>
            <Typography variant={TypographyVariants::H1}>
                { "Window Apis" }
            </Typography>
            <Typography>
                {"How to use the window() apis using web_sys"}
            </Typography>
            <GridMenu
                <WindowApisRoute>
                items={
                    vec!(
                        Item {
                        text: "Intersection Observer".to_string(),
                        route: WindowApisRoute::IntersectionObserver,
                    },

                )
                }
            />
        </div>
            <Switch<WindowApisRoute> render={switch} />
        </>
    }
}
