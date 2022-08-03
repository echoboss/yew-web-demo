use stylist::{style, Style};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::router::Route;

pub struct Hello {
    pub msg: String,
    pub style_sheet: Style,
    pub counter: i32,
}

pub enum Msg {
    Add,
    Sub,
}

impl Hello {
    fn style() -> Style {
        style!(
            r#"
                color: red;
            "#
        )
        .unwrap()
    }
}

impl Component for Hello {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            msg: "hello ly".to_owned(),
            style_sheet: Self::style(),
            counter: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                self.counter += 10;
                true
            }
            Msg::Sub => {
                self.counter -= 6;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1 class={self.style_sheet.clone()} >{format!("msg: {} count: {}",&self.msg,&self.counter)}</h1>
                <button onclick={ ctx.link().callback(|_| {Msg::Sub})}>{"-"}</button>
                <button onclick={ ctx.link().callback( |_| {Msg::Add})}>{"+"}</button>
                <br/>
                <Link<Route> to={Route::Home}>{"go home"} </Link<Route>>
            </>
        }
    }
}
