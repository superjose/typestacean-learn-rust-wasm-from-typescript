use leptos::*;

pub enum LeptosTypographyVariants {
    H1,
    P,
}

impl LeptosTypographyVariants {
    pub fn render(&self, cx: Scope, children: Children) -> impl IntoView {
        match self {
            LeptosTypographyVariants::H1 => {
                view! {cx, <h1 class="text-2xl">{children(cx)}</h1>}.into_view(cx)
            }
            LeptosTypographyVariants::P => view! {cx, <p>{children(cx)}</p>}.into_view(cx),
        }
    }
}

#[component]
pub fn leptos_typography(
    cx: Scope,
    variant: LeptosTypographyVariants,
    children: Children,
) -> impl IntoView {
    variant.render(cx, children)
}
