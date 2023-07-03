use leptos::*;

#[component]
pub fn button(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <button class="bg-slate-200 rounded-md"> "Hello" </button>
    }
}
