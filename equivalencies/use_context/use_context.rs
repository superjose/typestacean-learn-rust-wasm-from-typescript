#[derive(PartialEq, Debug, Properties)]
pub struct MangaOptionsContextProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MangaOptionsContextProvider)]
pub fn manga_options_context_provider(props: &MangaOptionsContextProps) -> Html {
    let state = use_reducer(MangaOptionsState::default);
    html! {
        <ContextProvider<MangaOptionsContext> context={state}>
            {props.children.clone()}
        </ContextProvider<MangaOptionsContext>>
    }
}

pub fn use_manga_options_context() -> impl Hook<Output = Option<UseReducerHandle<MangaOptionsState>>>
{
    use_context::<MangaOptionsContext>()
}
