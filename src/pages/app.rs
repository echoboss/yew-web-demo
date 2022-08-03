use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
