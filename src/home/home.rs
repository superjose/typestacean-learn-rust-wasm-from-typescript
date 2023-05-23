use yew::prelude::*;

use crate::components::{
    GridMenu::grid_menu::{GridMenu, Item},
    Typography::typography::{Typography, TypographyVariants},
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <Typography variant={TypographyVariants::H1}>
            { "Home 🤗" }
        </Typography>
        <GridMenu
            items={
                vec!(Item {
                    text: "Conditional Rendering".to_string(),
                    href: "/conditional-rendering".to_string(),
                })
            }
            />
        </>
    }
}
