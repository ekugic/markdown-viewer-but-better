mod components;
mod utils;

use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos::task::spawn_local;
use components::{file_selector::{FileSelector, load_file, log}, markdown_viewer::{MarkdownViewer, ViewMode}};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{KeyboardEvent, window};
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct EmptyArgs {}

#[component]
fn App() -> impl IntoView {
    let (content, set_content) = signal(String::new());
    let (file_path, set_file_path) = signal(None::<String>);
    let (view_mode, set_view_mode) = signal(ViewMode::Rendered);

    // Check for initial file from CLI on startup
    Effect::new(move |_| {
        spawn_local(async move {
            log("Checking for initial file from CLI...");
            set_content.set("Loading...".to_string());

            let args = serde_wasm_bindgen::to_value(&EmptyArgs {}).unwrap();
            let result = invoke("get_initial_file", args).await;

            match serde_wasm_bindgen::from_value::<Option<String>>(result) {
                Ok(Some(path)) => {
                    log(&format!("Loading initial file: {}", path));
                    set_content.set(format!("Loading file: {}", path));
                    load_file(path, set_content, set_file_path).await;
                }
                Ok(None) => {
                    log("No initial file provided");
                    set_content.set("".to_string());
                }
                Err(e) => {
                    let error_msg = format!("Error getting initial file: {:?}", e);
                    log(&error_msg);
                    set_content.set(error_msg);
                }
            }
        });
    });

    // Set up keyboard shortcut listener
    Effect::new(move |_| {
        if let Some(window) = window() {
            let document = window.document().unwrap();

            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: KeyboardEvent| {
                // Check for Ctrl+R
                if event.ctrl_key() && event.key() == "r" {
                    event.prevent_default();
                    // Toggle view mode
                    set_view_mode.update(|mode| {
                        *mode = match *mode {
                            ViewMode::Raw => ViewMode::Rendered,
                            ViewMode::Rendered => ViewMode::Raw,
                        };
                    });
                }
            }) as Box<dyn FnMut(_)>);

            document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();

            // Keep closure alive
            closure.forget();
        }
    });

    let mode_text = move || {
        match view_mode.get() {
            ViewMode::Raw => "Raw Mode",
            ViewMode::Rendered => "Rendered Mode",
        }
    };

    let file_name_display = move || {
        file_path.get().as_ref().map(|p| {
            std::path::Path::new(p.as_str())
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(p.as_str())
                .to_string()
        })
    };

    view! {
        <div class="app-container">
            <div class="toolbar">
                <FileSelector set_content=set_content set_file_path=set_file_path />
                <div class="mode-indicator">{mode_text}</div>
                {move || file_name_display().map(|name| view! {
                    <span class="file-name">{name}</span>
                })}
            </div>
            <MarkdownViewer content=content view_mode=view_mode />
        </div>
    }
}

pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    hydrate();
}
