mod components;
mod utils;

use leptos::prelude::*;
use leptos::mount::mount_to_body;
use components::{file_selector::FileSelector, markdown_viewer::{MarkdownViewer, ViewMode}};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, window};

#[component]
fn App() -> impl IntoView {
    let (content, set_content) = signal(String::new());
    let (file_path, set_file_path) = signal(None::<String>);
    let (view_mode, set_view_mode) = signal(ViewMode::Rendered);

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
