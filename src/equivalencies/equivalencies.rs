use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::grid_menu::grid_menu::{GridMenu, Item};
use crate::components::typography::typography::{Typography, TypographyVariants};
use crate::equivalencies::conditional_rendering::conditional_rendering::{
    ConditionalRendering, ConditionalVariant,
};
use crate::equivalencies::logging::log::LogComponent;
use crate::equivalencies::onclick::onclick::OnClickComponent;
use crate::utils::routes::Route;

use super::use_reducer::user_reducer::UseReducerComponent;

#[derive(Clone, Routable, PartialEq)]
enum EquivalenciesRoute {
    #[at("/conditional_rendering")]
    ConditionalRendering,
    #[at("/logging")]
    Logging,
    #[at("/onclick")]
    OnClick,
    #[at("/use-reducer")]
    UseReducer,
}

fn switch(routes: EquivalenciesRoute) -> Html {
    match routes {
        EquivalenciesRoute::ConditionalRendering => {
            html! { <ConditionalRendering variant={ConditionalVariant::Primary} /> }
        }
        EquivalenciesRoute::Logging => html! { <LogComponent /> },
        EquivalenciesRoute::OnClick => html! { <OnClickComponent /> },
        EquivalenciesRoute::UseReducer => html! { <UseReducerComponent /> },
    }
}

#[function_component(Equivalencies)]
pub fn equivalencies() -> Html {
    html! {
        <>
        <div>
            <Typography variant={TypographyVariants::H1}>
                { "Equivalencies" }
            </Typography>
             <GridMenu
                <Route>
                items={
                    vec!(Item {
                        text: "Equivalencies".to_string(),
                        route: Route::Equivalencies,
                    })
                }
            />
        </div>
            <Switch<EquivalenciesRoute> render={switch} />
        </>
    }
}
