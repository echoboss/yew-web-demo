use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::pages::hello::Hello;
use crate::pages::home::Home;
use crate::pages::movie::Movie;
use crate::pages::test::Test;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
    #[at("/test")]
    Test,
    #[at("/movie/:id")]
    Movie { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <WithDispatch<Home> />
        },
        Route::Test => html! {
            <Test />
        },
        Route::Movie { id } => html! {
            <Movie id={id.to_owned()}/>
        },
        Route::Hello => html! {
            <Hello />
        },
        Route::NotFound => html! {
            <h1>{"404"}</h1>
        },
    }
}
