use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChangedBy {
    Navigation,
    Manually,
}

impl From<ChangedBy> for JsValue {
    fn from(value: ChangedBy) -> Self {
        match value {
            ChangedBy::Navigation => JsValue::from("navigation"),
            ChangedBy::Manually => JsValue::from("manually"),
        }
    }
}

#[function_component(LogComponent)]
pub fn log_component() -> Html {
    let changed_by = ChangedBy::Navigation;
    // Then you can, in your component:
    log!("Hello TypeStacean {}!", changed_by);
    html! {
        <p>{"Hello world"}</p>
    }
}
