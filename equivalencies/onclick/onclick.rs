// Fictional Imports
use component::{Icon, IconButton};
use states::manga::{use_manga_context, MangaAction, MangaState};
use yew::prelude::*;

#[function_component(OnClickComponent)]
pub fn on_click_component() -> Html {
    let state = use_manga_context().unwrap();

    let go_prev = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(MangaAction::Prev);
        })
    };

    html! {
        <IconButton
                class="hidden sm:block"
                on_click={go_prev_chapter}
                icon={Icon::DoubleLeftArrow}
                disabled={prev_chapter_disabled}
            />
    }
}
