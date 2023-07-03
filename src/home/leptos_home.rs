use leptos::*;

use crate::components::typography::leptos_typography::{
    LeptosTypography, LeptosTypographyVariants,
};

// This will be rendered as Home
#[component]
pub fn home(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <LeptosTypography
            variant={LeptosTypographyVariants::H1}
        >
        "Hello 🤗"
        </LeptosTypography>
    }
}
