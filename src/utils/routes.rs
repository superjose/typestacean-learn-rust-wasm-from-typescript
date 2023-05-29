use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/equivalencies")]
    EquivalenciesRoot,
    #[at("/equivalencies/*")]
    Equivalencies,
    #[at("/window")]
    WindowApisRoot,
    #[at("/window/*")]
    WindowApis,
}
