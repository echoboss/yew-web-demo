use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::pages::router::Route;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Movie)]
pub fn movie(props: &Props) -> Html {

    let history = use_history().unwrap();
    let onclick = Callback::once(move|_|{
        history.push(Route::Home);
    });
    
    html! {
        <div>
            <h1>{format!("mv: {}",&props.id)}</h1>
            <button {onclick}>{"go home"}</button>
        </div>
    }
}
