use crate::components::typography::typography::{Typography, TypographyVariants};
use yew::prelude::*;

#[function_component(OnClickComponent)]
pub fn on_click_component() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <>
            <button {onclick}>{"Increase Counter"}</button>
            <Typography variant={TypographyVariants::P}>
                <b>{ "Current value: " }</b>
                    { *counter }
            </Typography>
        </>
    }
}
