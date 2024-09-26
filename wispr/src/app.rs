#![allow(non_snake_case)]

use chrono::prelude::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub fn App() -> Element {
    let mut name = use_signal(|| String::new());
    let mut greet_msg = use_signal(|| String::new());

    let greet = move |_: FormEvent| async move {
        if name.read().is_empty() {
            return;
        }

        let name = name.read();
        let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        let new_msg = invoke("greet", args).await.as_string().unwrap();
        greet_msg.set(new_msg);
    };

    let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    let date = format!("{}", utc.format("%Y-%m-%d"));

    rsx! {
        link { rel: "stylesheet", href: "styles.css" }
        main {
            class: "window",

            div {
                class: "topbar",
                div {
                    class: "date",
                    p { class:"logo", "\u{280B}" }
                    button { class: "date-btn", r#type: "submit", "< " }
                    p { class: "date-text", "{date}" }
                    button { class: "date-btn", r#type: "submit", " >" }
                }
            }
            form {
                class: "textarea-form",
                div {
                    class: "textarea-div",
                    p { class: "textarea-title", "todo;"}
                    textarea { class: "textarea", id: "textarea-todo" }
                }
                div {
                    class: "textarea-div",
                    p { class: "textarea-title", "note;"}
                    textarea { class: "textarea", id: "textarea-note" }
                }
            }
        }
    }

    //rsx! {
    //    link { rel: "stylesheet", href: "styles.css" }
    //    main {
    //        class: "container",
    //        div {
    //            class: "row",
    //            a {
    //                href: "https://tauri.app",
    //                target: "_blank",
    //                img {
    //                    src: "/tauri.svg",
    //                    class: "logo tauri",
    //                     alt: "Tauri logo"
    //                }
    //            }
    //            a {
    //                href: "https://dioxuslabs.com/",
    //                target: "_blank",
    //                img {
    //                    src: "/dioxus.png",
    //                    class: "logo dioxus",
    //                    alt: "Dioxus logo"
    //                }
    //            }
    //        }
    //
    //        p { "Click on the Tauri and Dioxus logos to learn more." }
    //
    //        form { form {
    //            class: "row",
    //            onsubmit: greet,
    //            input {
    //                id: "greet-input",
    //                placeholder: "Enter a name...",
    //                value: "{name}",
    //                oninput: move |event| name.set(event.value())
    //            }
    //            button { r#type: "submit", "Greet" }
    //        }

    //            class: "row",
    //            onsubmit: greet,
    //            input {
    //                id: "greet-input",
    //                placeholder: "Enter a name...",
    //                value: "{name}",
    //                oninput: move |event| name.set(event.value())
    //            }
    //            button { r#type: "submit", "Greet" }
    //        }
    //
    //        p {
    //            b { "{greet_msg}" }
    //        }
    //    }
    //}
}
