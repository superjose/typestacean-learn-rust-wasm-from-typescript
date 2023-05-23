use gloo::console::log;
use wasm_bindgen::JsValue;

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

#[function_component(MyComponent)]
fn my_component() -> Html {
    let changed_by = ChangedBy::Navigation;
    // Then you can, in your component:
    log!("Navigation is {}", changed_by);
    html! {
        <p>Hello world</p>
    }
}
