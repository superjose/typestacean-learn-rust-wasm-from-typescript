use yew::prelude::*;

#[derive(PartialEq)]
pub enum TypographyVariants {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    p,
}

impl Default for TypographyVariants {
    fn default() -> Self {
        TypographyVariants::p
    }
}

impl TypographyVariants {
    pub fn render(&self, children: Children) -> Html {
        match self {
            TypographyVariants::H1 => html! { <h1>{ children }</h1> },
            TypographyVariants::H2 => html! { <h2>{ children }</h2> },
            TypographyVariants::H3 => html! { <h3>{ children }</h3> },
            TypographyVariants::H4 => html! { <h4>{ children }</h4> },
            TypographyVariants::H5 => html! { <h5>{ children }</h5> },
            TypographyVariants::H6 => html! { <h6>{ children }</h6> },
            TypographyVariants::p => html! { <p>{ children }</p> },
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct TypographyProps {
    #[prop_or_default()]
    variant: TypographyVariants,
    #[prop_or_default()]
    children: Children,
}

#[function_component(Typography)]
pub fn typography(prop: &TypographyProps) -> Html {
    prop.variant.render(prop.children.clone())
}
