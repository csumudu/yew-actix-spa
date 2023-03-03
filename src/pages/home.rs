use std::ops::Deref;

use gloo::console::log;
use reqwasm::http::Request;
use serde::Deserialize;
use serde::Serialize;
use yew::Html;

use yew::prelude::*;

#[derive(Serialize,Clone, Deserialize, PartialEq)]
pub struct RustLogos {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

#[function_component]
pub fn Home() -> Html {
    let logo_state = use_state(|| vec![]);

    let onclick = {
        let state = logo_state.clone();
        Callback::from(move |_| {
            log!("Clicked");

            let state = state.clone();

            wasm_bindgen_futures::spawn_local( async move {
                let res = Request::get("http://localhost:4400/api/logos")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<RustLogos>>()
                    .await
                    .unwrap();

                log!(serde_json::to_string_pretty(&res).unwrap());
                state.set(res);

            })
        })
    };

    html! {
      
       <div>
       <div class="pb-2">

       <button {onclick} class="btn btn-primary">{"Get Logos"}</button></div>
       <div>
        {logos_to_html(logo_state.to_vec())}
        </div>
        </div>
    }
}


fn logos_to_html(logos: Vec<RustLogos>) -> Html {
    let items = logos.iter()
        .map(move |a| html! {
            
            <div class={classes!(format!("itm-img-{}",&a.id))}>
            <div class="card">
                <div class="img-con" style={format!("background-image: url(http://localhost:4400/{})",&a.image_path.replace("./", ""))}>{""}</div>

                <div class="card-body">
                    <h5 class="card-title">{&a.name}</h5>
                    <p class="card-text">{"Rust Image"}</p>
                    <a href="#" class="btn btn-primary">{"Show"}</a>
                </div>
            </div>
        </div>
        
        })
        .collect::<Html>();


        html!{
            <div class="gallery">
           
{items}
            

         
        </div>
        }
}
