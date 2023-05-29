use yew::prelude::*;

use crate::{
    components::typography::typography::{Typography, TypographyVariants},
    window_apis::intersection_observer::div_intersect::DivIntersect,
};

#[function_component(IntersectionObserver)]
pub fn intersection_observer() -> Html {
    html! {
        <>
        <Typography variant={TypographyVariants::H1}>{"Intersection Observer"}</Typography>
        <Typography variant={TypographyVariants::P}>{"Scroll down and open the console"}</Typography>

            {(0..=10).map(|i| {
                html! {
                    <DivIntersect index={i.clone()} />
                }
            }).collect::<Html>()}
        </>
    }
}
