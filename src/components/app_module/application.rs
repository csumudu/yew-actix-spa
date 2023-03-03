use crate::{
    components::app_module::header::Header,
    roures::{switch_routes, AppRoutes},
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="container">
        <Header/>
        <div class="inner-routes">
            <BrowserRouter>
                <Switch<AppRoutes> render={switch_routes} />
            </BrowserRouter>
        </div>
        </div>
    }
}
