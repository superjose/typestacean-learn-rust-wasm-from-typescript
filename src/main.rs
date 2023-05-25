use crate::equivalencies::equivalencies::Equivalencies;
use crate::home::home::Home;
use crate::utils::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod equivalencies;
mod home;
mod utils;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Equivalencies | Route::EquivalenciesRoot => html! { <Equivalencies /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
