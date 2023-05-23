use yew::prelude::*;

enum ConditionalVariant {
    Primary,
    Secondary,
}

#[derive(Properties, Clone, PartialEq)]
struct ConditionalRenderingProps {
    variant: ConditionalVariant,
}

#[function_component(ConditionalRendering)]
pub fn conditional_rendering(props: &ConditionalRenderingProps) {
    let variant = props.variant;
    html! {
        <div>

            {if variant == ConditionalVariant::Primary {
                html! {
                    <p>Primary</p>
                }
            } else {
                html! {
                    <p>Secondary</p>
                }
            }}
        </div>
    }
}
