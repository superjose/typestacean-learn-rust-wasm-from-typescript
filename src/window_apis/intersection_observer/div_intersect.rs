use gloo::{console::log, utils::window};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use yew::prelude::*;

use web_sys::{
    HtmlDivElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};

/**
* For this file, we enable the following features in cargo.toml:
*  'IntersectionObserver',
   'IntersectionObserverInit',
   'IntersectionObserverEntry',
*/

#[derive(Clone, Properties, PartialEq)]
pub struct DivIntersectProps {
    pub index: i8,
}

#[function_component(DivIntersect)]
pub fn div_intersect(props: &DivIntersectProps) -> Html {
    let index = props.index;
    let div_ref = use_node_ref();

    {
        let div = div_ref.clone();
        use_effect(move || {
            let mut options = IntersectionObserverInit::new();
            let is_phone = window()
                .match_media("(max-width: 320px)")
                .unwrap()
                .unwrap()
                .matches();
            let threshold = if is_phone { 0.7 } else { 0.6 };
            options.threshold(&JsValue::from(threshold));

            let div = div.cast::<HtmlDivElement>().expect("div not set");

            // This is a closure that will be called when the intersection observer
            // detects that the div is intersecting the viewport.
            let callback = Closure::wrap(Box::new(
                move |entries: Vec<JsValue>, _observer: IntersectionObserver| {
                    for entry in entries {
                        let entry = IntersectionObserverEntry::from(entry);
                        let is_intersecting = entry.is_intersecting();

                        if is_intersecting {
                            log!(format!("intersecting div # {}", index.clone()));
                        }
                    }
                },
            )
                as Box<dyn FnMut(Vec<JsValue>, IntersectionObserver)>);

            let observer =
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                    .unwrap();

            observer.observe(&div);

            move || {
                callback.forget();
                observer.unobserve(&div);
            }
        });
    }

    html! {
        <div ref={div_ref} style="height: 500px; width: 500px; max-width:100%; background-color: red; margin: 10px;">
            {index}
        </div>
    }
}
