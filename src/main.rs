use crate::equivalencies::equivalencies::Equivalencies;
use crate::home::home::Home;
use crate::utils::routes::Route;
use window_apis::window_apis::WindowApis;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod equivalencies;
mod home;
mod other;
mod utils;
mod window_apis;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Equivalencies | Route::EquivalenciesRoot => html! { <Equivalencies /> },
        Route::WindowApis | Route::WindowApisRoot => html! { <WindowApis /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn dioxus_app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn dioxus_app() {

}

fn main() {
    yew::Renderer::<App>::new().render();
}
