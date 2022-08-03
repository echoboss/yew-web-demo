use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::pages::counter::NumCounter;
use crate::pages::router::Route;

use crate::pages::store::{init, AppStore};

use crate::pages::display::DisplayCounter;

pub struct Home {
    _dispatch: Dispatch<BasicStore<AppStore>>,
}

impl Component for Home {
    type Message = ();

    type Properties = DispatchProps<BasicStore<AppStore>>;

    fn create(_ctx: &Context<Self>) -> Self {
        let _dispatch = init();
        Self { _dispatch }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        
        html! {
            <div>
                <WithDispatch<NumCounter>/>
                
                <WithDispatch<DisplayCounter>/>

                <h1>{"Home"}</h1>
                <Link<Route> to={Route::Hello}> {"go to hello"}</Link<Route>>
                <br/>
                <Link<Route> to={Route::Test}>{"go to test"}</Link<Route>>
                <br/>
                <Link<Route> to={Route::Movie{id: "ly".to_string()}}>{"go to movie"}</Link<Route>>
            </div>
        }
    }
}
