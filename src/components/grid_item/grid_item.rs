use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct GridItemProps {
    pub children: Children,
    #[prop_or_default(false)]
    pub active: bool,
}

#[function_component(GridItem)]
pub fn grid_item(props: &GridItemProps) -> Html {
    let active = if props.active { "bg-purple-500" } else { "" };

    html! {
        <div
          class={
            format!("bg-slate-400 hover:bg-slate-500 hover:cursor-pointer p-4 {}", active)
          }
        >
          {props.children.clone()}
        </div>
    }
}
