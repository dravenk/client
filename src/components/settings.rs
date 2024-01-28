use log;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::html;
use yew::prelude::*;

use yew::{Callback, Html};

// https://github.com/tauri-apps/tauri/issues/8073#issuecomment-1773530639
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GenerateOptions<'a> {
    strength: &'a str,
}

#[function_component(Settings)]
pub fn settings() -> Html {
    let input_node_ref = use_node_ref();
    let words = use_state(|| vec![String::new(); 24]);
    let strength = use_state(|| String::new());
    let show = use_state(|| true);

    {
        let strength = strength.clone();
        let selected_strength = strength.clone();
        use_effect_with(selected_strength, move |_| {
            log::info!("I'm a effect!");
            spawn_local(async move {});
            || {}
        });
    }
    let show_clone = show.clone();

    let callback = {
        log::info!("I'm a onchange!!");
        let strength_clone = strength.clone();
        let words_clone = words.clone();

        let e = move |e: MouseEvent| {
            e.prevent_default();
            log::info!("show_clone {:?}", show_clone.clone());

            let show = show_clone.clone();
            if *show {
                log::info!("I'm a show_clone!");
                show.set(false);
            } else {
                log::info!("I'm a show_clone!");
                show.set(true);
            }

            if !*show_clone {
                log::info!("show_clone {:?}, return", show_clone.clone());
                words_clone.set(vec![String::new(); 24]);
                return;
            }

            let strength = strength_clone.clone();
            let words = words_clone.clone();

            spawn_local(async move {
                let args = to_value(&GenerateOptions {
                    strength: &*strength,
                })
                .unwrap();
                let new_msg = invoke("generage_key", args).await.as_string().unwrap();
                let w = new_msg.split(" ").collect::<Vec<&str>>();
                words.set(w.iter().map(|s| s.to_string()).collect::<Vec<String>>());
            });
        };

        Callback::from(e)
    };
    let show_value = if *show.clone() {
        "Show".to_string()
    } else {
        "Hide".to_string()
    };
    let show_words = if *show.clone() {
        "hidden"
    } else {
        "flex flex-wrap"
    };
    html! {
         <div class="">
            <div class="">
             <button class="h-14 mx-8 my-4 px-4 flex justify-center items-center
                focus:text-orange-500  rounded-2xl bg-gray-200 hover:bg-gray-300"
                 onclick={ callback }>
                { show_value }
                </button>
            </div>
            <div class={show_words}>
               {
                    for (1..25).map(|i| {
                        let words = words.clone();
                        let word = words[i-1].clone();
                        let word_clone = word.to_owned().clone();
                        let input_id = "input-".to_string() + i.to_string().as_str();
                        let input_node_ref = input_node_ref.clone();

                        html! {
                            <div class="flex items-center m-1 p-2 h-12 w-36" >
                                <div class="number mr-1.5 w-4">{ i.clone() }</div>
                                <input
                                class="w-full p-2 rounded-xl"
                                disabled={true}
                                placeholder={ word_clone.clone()}
                                id={ input_id }
                                    type="text"
                                    value={word_clone.clone()}
                                    ref={input_node_ref}
                                />
                            </div>
                        }
                    })
               }
            </div>
        </div>
    }
}
