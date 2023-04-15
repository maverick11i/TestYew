use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(routers: Route) -> Html {
    match routers {
        Route::Home => {
            html! {
                <home::Home />
            }
        }
    }
}
