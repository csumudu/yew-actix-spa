use yew::prelude::*;
use yew::Html;
use yew_router::prelude::*;

use crate::pages::counter::Counter;
use crate::pages::home::Home;

#[derive(Debug,Copy, Clone, PartialEq, Routable)]
pub enum AppRoutes {
    #[at("/")]
    HOME,
    #[at("/counter")]
    COUNTER,
}

pub fn switch_routes(route: AppRoutes) -> Html {
    match route {
        AppRoutes::HOME => html! {<Home/>},
        AppRoutes::COUNTER => html! {<Counter/>},
    }
}
