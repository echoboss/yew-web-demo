use yew::prelude::*;

use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub body: Option<String>,
    pub submit: Callback<String>,
    pub on_change: Callback<Event>,
}

#[styled_component(Layout)]
pub fn layout(props: &Props) -> Html {
    let style_sheet = style!(
        r#"
            color: red;
            font-size: 20px;
            text-align: center;
        "#
    )
    .unwrap();

    props.submit.emit("hello!".to_string());

    html! {
        <>
            <h1 class={style_sheet} >{props.body.as_ref().unwrap_or(&"hi layout !".to_string())}</h1>
            <input type="text" onchange={&props.on_change} />
        </>
    }
}
