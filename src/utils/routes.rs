use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/equivalencies")]
    Equivalencies,
    #[not_found]
    #[at("/404")]
    NotFound,
}
