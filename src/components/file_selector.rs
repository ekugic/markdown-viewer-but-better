use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub async fn load_file(path: String, set_content: WriteSignal<String>, set_file_path: WriteSignal<Option<String>>) {
    set_file_path.set(Some(path.clone()));

    let read_args = serde_wasm_bindgen::to_value(&ReadFileArgs { path: path.clone() }).unwrap();
    log("About to read file content");
    let content_result = invoke("read_file_content", read_args).await;

    match serde_wasm_bindgen::from_value::<String>(content_result) {
        Ok(content) => {
            log("File content loaded successfully");
            set_content.set(content);
        }
        Err(e) => {
            let error_msg = format!("Error reading file {}: {:?}", path, e);
            log(&error_msg);
            set_content.set(error_msg);
        }
    }
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
        log("Button clicked!");
        spawn_local(async move {
            log("Inside spawn_local");

            // Call Tauri command to select file
            let args = serde_wasm_bindgen::to_value(&EmptyArgs {}).unwrap();
            log("About to invoke select_markdown_file");
            let result = invoke("select_markdown_file", args).await;
            log("Invoke completed");

            // Convert result to String
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(path) => {
                    log(&format!("File selected: {}", path));
                    set_file_path.set(Some(path.clone()));

                    // Read file content
                    let read_args = serde_wasm_bindgen::to_value(&ReadFileArgs { path: path.clone() }).unwrap();
                    log("About to read file content");
                    let content_result = invoke("read_file_content", read_args).await;

                    match serde_wasm_bindgen::from_value::<String>(content_result) {
                        Ok(content) => {
                            log("File content loaded successfully");
                            set_content.set(content);
                        }
                        Err(e) => {
                            let error_msg = format!("Error reading file {}: {:?}", path, e);
                            log(&error_msg);
                            set_content.set(error_msg);
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Error selecting file: {:?}", e);
                    log(&error_msg);
                    set_content.set(error_msg);
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
