use yew::prelude::*;

use crate::{
    components::{
        grid_menu::grid_menu::{GridMenu, Item},
        typography::typography::{Typography, TypographyVariants},
    },
    utils::routes::Route,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <Typography variant={TypographyVariants::H1}>
            { "Home 🤗" }
        </Typography>
        <GridMenu
            <Route>
            items={
                vec!(Item {
                    text: "Equivalencies".to_string(),
                    route: Route::Equivalencies,
                })
            }
            />
        </>
    }
}
