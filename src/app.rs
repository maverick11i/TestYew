use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::route;

/// アプリのエンドポイント
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<route::Route> render={route::switch} />
        </BrowserRouter>
    }
}
