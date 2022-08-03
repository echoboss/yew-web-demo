use gloo::console::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{layout::Layout, router::Route};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[function_component(Test)]
pub fn test() -> Html {
    let u = use_state(|| User {
        name: "".to_string(),
        age: 0,
    });

    let counter = use_state(|| 0);

    use_effect(|| {
        || {
            log!("use effect.");
        }
    });

    let name = "ly";
    log!("hello ", name);
    let user = User {
        name: "ly".to_string(),
        age: 18,
    };
    if name.eq("ly") {
        log!("user:", serde_json::to_string(&user).unwrap());
    }
    let submit = Callback::from(move |value: String| {
        log!("submit:", value);
    });
    let u1 = u.clone();
    let on_change = Callback::from(move |e: Event| {
        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
        log!("input: ", target.value());
        u1.clone().set(User {
            name: target.value().to_string(),
            age: 18,
        });
    });

    let u2 = u.clone();
    let add_counter = counter.clone();
    let sub_counter = counter.clone();
    html! {
        <>
            <Link<Route> to={Route::Home}>{"go to home"}</Link<Route>>
            if ! &u.name.is_empty() {
                <p>{&u.name}</p>
            }

            <p>{&*counter}</p>

            <button onclick={Callback::from(move |_| {
                log!("add 1");
                add_counter.set(*add_counter + 10);
            })}>{"+"}</button>
            <button onclick={Callback::from(move |_| {
                sub_counter.clone().set(*sub_counter - 3);
            })}>{"-"}</button>

            <Layout submit={submit} on_change={on_change} />

            <button onclick={move |_|{
                log!(&u2.name);
                wasm_bindgen_futures::spawn_local(async {
                    let resp = reqwasm::http::Request::get("http://bing.com")
                        .send()
                        .await
                        .unwrap();
                    log!(resp.text().await.unwrap());
                });
            }}>{"click"}</button>
        </>
    }
}
