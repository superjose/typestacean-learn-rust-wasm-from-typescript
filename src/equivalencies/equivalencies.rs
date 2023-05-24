use yew::prelude::*;
use yew_router::prelude::*;

use crate::equivalencies::conditional_rendering::conditional_rendering::{
    ConditionalRendering, ConditionalVariant,
};
use crate::equivalencies::logging::log::LogComponent;
use crate::equivalencies::onclick::onclick::OnClickComponent;

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
            <h1>{ "Equivalencies" }</h1>
        </div>
            <Switch<EquivalenciesRoute> render={switch} />
        </>
    }
}
