use yew::prelude::*;
use yewdux::prelude::*;

use crate::pages::store::AppStore;

pub struct NumCounter {
    _dispatch: DispatchProps<BasicStore<AppStore>>,
}

impl Component for NumCounter {
    type Message = ();

    type Properties = DispatchProps<BasicStore<AppStore>>;

    fn create(ctx: &Context<Self>) -> Self {
        let _dispatch = ctx.props().dispatch().clone();
        Self { _dispatch }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().state().count;
        let onclick = ctx
            .props()
            .dispatch()
            .reduce_callback(|state| state.count += 1);
        //gloo::storage::LocalStorage::set("yew_key", count).expect("msg");
        html! {
            <>
                <h1>{format!("num: {}",count)}</h1>
                <button onclick={onclick}>{"+"} </button>
            </>
        }
    }
}
