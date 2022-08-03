use yew::prelude::*;
use yewdux::prelude::*;

use crate::pages::store::AppStore;

pub struct DisplayCounter {
    _dispatch: DispatchProps<BasicStore<AppStore>>,
}

impl Component for DisplayCounter {
    type Message = ();

    type Properties = DispatchProps<BasicStore<AppStore>>;

    fn create(ctx: &Context<Self>) -> Self {
        let _dispatch = ctx.props().dispatch().clone();
        Self { _dispatch }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().state().count;
        html! {
            <h1>{format!("display: {}",count)}</h1>
        }
    }
}
