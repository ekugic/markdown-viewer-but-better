use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct EmptyArgs {}

#[derive(Serialize, Deserialize)]
struct ReadFileArgs {
    path: String,
}

#[component]
pub fn FileSelector(
    set_content: WriteSignal<String>,
    set_file_path: WriteSignal<Option<String>>,
) -> impl IntoView {
    let select_file = move |_| {
        spawn_local(async move {
            // Call Tauri command to select file
            let args = serde_wasm_bindgen::to_value(&EmptyArgs {}).unwrap();
            let result = invoke("select_markdown_file", args).await;

            // Convert result to String
            if let Ok(path) = serde_wasm_bindgen::from_value::<String>(result) {
                set_file_path.set(Some(path.clone()));

                // Read file content
                let read_args = serde_wasm_bindgen::to_value(&ReadFileArgs { path: path.clone() }).unwrap();
                let content_result = invoke("read_file_content", read_args).await;

                if let Ok(content) = serde_wasm_bindgen::from_value::<String>(content_result) {
                    set_content.set(content);
                } else {
                    set_content.set(format!("Error reading file: {}", path));
                }
            }
        });
    };

    view! {
        <button on:click=select_file>
            "Open File"
        </button>
    }
}
