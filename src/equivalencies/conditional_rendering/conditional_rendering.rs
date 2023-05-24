use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ConditionalVariant {
    Primary,
    Secondary,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ConditionalRenderingProps {
    pub variant: ConditionalVariant,
}

#[function_component(ConditionalRendering)]
pub fn conditional_rendering(props: &ConditionalRenderingProps) -> Html {
    let variant = &props.variant;
    html! {
        <div>
            {
                match variant {
                    ConditionalVariant::Primary => html! {
                        <p>{"Primary"}</p>
                    },
                    ConditionalVariant::Secondary => html! {
                        <p>{"Secondary"}</p>
                    },
                }
            }


        </div>
    }
}
