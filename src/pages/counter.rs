use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct School {
    name: String,
    age: i32,
}

#[function_component]
pub fn Counter() -> Html {
    let counter = use_state(|| 0);

    let styles = style!(
        r#"
            p {
                color:red
            }

            .test-ul{
                color:orange
            }

            .test-li{
                color:green
            }
        
        "#
    )
    .unwrap();

    log!("My name is ", *counter);

    let x = School {
        name: "test".to_owned(),
        age: 23,
    };

    let schools = vec![
        School {
            age: 10,
            name: "One".to_owned(),
        },
        School {
            age: 110,
            name: "Two".to_owned(),
        },
        School {
            age: 120,
            name: "Three".to_owned(),
        },
    ];

    log!("Test-->", serde_json::to_string_pretty(&x).unwrap());

    let increment = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let decrement = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    html! {

        <div class={styles}>
        <p>{"Counter    : "}{*counter}</p>
        <button onclick={increment} class="btn btn-primary">{"Increment"}</button>
        <button onclick={decrement} class="btn btn-primary">{"Decrement"}</button>

        <ul class="test-ul">
        {schools_to_html(schools)}
        </ul>
        </div>
    }
}

fn schools_to_html(sch: Vec<School>) -> Html {
    sch.iter()
        .map(move |a| html! {<li class="test-li">{&a.name}</li>})
        .collect::<Html>()
}
