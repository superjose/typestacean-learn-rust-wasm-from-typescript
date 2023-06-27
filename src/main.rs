use crate::equivalencies::equivalencies::Equivalencies;
use crate::home::home::Home;
use crate::utils::routes::Route;
use dioxus::prelude::*;
use leptos::{mount_to_body, view};
use std::str::FromStr;
use wasm_framework::WasmFramework;
use window_apis::window_apis::WindowApis;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod env;
mod equivalencies;
mod home;
mod other;
mod utils;
mod wasm_framework;
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
fn yew_app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

// create a component that renders a div with the text "Hello, world!"
fn dioxus_app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}

fn leptos_app() {
    mount_to_body(|cx| view! { cx,  <p>"Hello, world!"</p> })
}

fn main() {
    let framework = WasmFramework::from_str(env::APP_WASM_FRAMEWORK).unwrap_or_else(|_| {
        println!(
            "There was a problem while parsing the env {}",
            env::APP_WASM_FRAMEWORK
        );
        println!("Defaulting to yew");
        return WasmFramework::Yew;
    });

    match framework {
        WasmFramework::Yew => {
            yew::Renderer::<App>::new().render();
        }
        WasmFramework::Dioxus => {
            dioxus_web::launch(dioxus_app);
        }
        WasmFramework::Leptos => {
            leptos_app();
        }
    }
}
