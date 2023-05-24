use std::{cmp::max, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Action {
    Increase,
    Decrease,
    SetValue(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentState {
    value: u32,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl Reducible for ComponentState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Increase => Self {
                value: self.value + 1,
            }
            .into(),
            Action::Decrease => Self {
                value: max(self.value - 1, 0),
            }
            .into(),
            Action::SetValue(value) => Self { value }.into(),
        }
    }
}

#[function_component(UseReducerComponent)]
pub fn use_reducer_component() -> Html {
    let state = use_reducer(ComponentState::default);

    let increase = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::Increase))
    };

    let decrease = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::Decrease))
    };

    let handle_change = {
        let state = state.clone();
        move |e: Event| {
            e.prevent_default();
            let val = e
                .target()
                .unwrap()
                // This needs to be turned on in web-sys Cargo.toml
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
            let to_number = val.parse::<u32>();
            if let Ok(val) = to_number {
                state.dispatch(Action::SetValue(val));
            }
        }
    };

    html! {
        <>
            <div>
                <h1>{ format!("Value is {}", &state.value) }</h1>
            </div>
            <div>
                <button onclick={increase}>{ "Increase" }</button>
                <button onclick={decrease}>{ "Decrease" }</button>
                <input type="number" value={state.value.to_string()} onchange={handle_change} />
            </div>
        </>
    }
}
