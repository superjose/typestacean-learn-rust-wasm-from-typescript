use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::grid_menu::grid_menu::{GridMenu, Item};
use crate::components::typography::typography::{Typography, TypographyVariants};
use crate::equivalencies::conditional_rendering::conditional_rendering::{
    ConditionalRendering, ConditionalVariant,
};
use crate::equivalencies::logging::log::LogComponent;
use crate::equivalencies::onclick::onclick::OnClickComponent;

use super::use_reducer::user_reducer::UseReducerComponent;

#[derive(Clone, Routable, PartialEq)]
enum EquivalenciesRoute {
    #[at("/equivalencies/conditional-rendering")]
    ConditionalRendering,
    #[at("/equivalencies/conditional-rendering-variant-2")]
    ConditionalRenderingVariant2,
    #[at("/equivalencies/logging")]
    Logging,
    #[at("/equivalencies/onclick")]
    OnClick,
    #[at("/equivalencies/use-reducer")]
    UseReducer,
}

fn switch(routes: EquivalenciesRoute) -> Html {
    match routes {
        EquivalenciesRoute::ConditionalRendering => {
            html! { <ConditionalRendering variant={ConditionalVariant::Primary} /> }
        }
        EquivalenciesRoute::ConditionalRenderingVariant2 => {
            html! { <ConditionalRendering variant={ConditionalVariant::Secondary} /> }
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
                <EquivalenciesRoute>
                items={
                    vec!(
                        Item {
                        text: "Conditional Rendering".to_string(),
                        route: EquivalenciesRoute::ConditionalRendering,
                    },
                        Item {
                        text: "Conditional Rendering Variant 2".to_string(),
                        route: EquivalenciesRoute::ConditionalRenderingVariant2,
                    },
                        Item {
                        text: "Logging".to_string(),
                        route: EquivalenciesRoute::Logging,
                    },
                        Item {
                        text: "Conditional Rendering".to_string(),
                        route: EquivalenciesRoute::OnClick,
                    },
                        Item {
                        text: "Conditional Rendering".to_string(),
                        route: EquivalenciesRoute::UseReducer,
                    },

                )
                }
            />
        </div>
            <Switch<EquivalenciesRoute> render={switch} />
        </>
    }
}
