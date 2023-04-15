use wasm_bindgen::JsCast;
use yew::prelude::*;

pub struct Home {
    data: String,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: "TimeSnap new application!".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{ self.data.clone() }</p>
            </>
        }
    }
}
