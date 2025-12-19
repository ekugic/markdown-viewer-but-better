use leptos::prelude::*;
use leptos::either::Either;
use crate::utils::markdown::markdown_to_html;

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Raw,
    Rendered,
}

#[component]
pub fn MarkdownViewer(
    content: ReadSignal<String>,
    view_mode: ReadSignal<ViewMode>,
) -> impl IntoView {
    view! {
        <div class="content-area">
            {move || {
                let current_content = content.get();
                let is_empty = current_content.is_empty();
                let is_raw = view_mode.get() == ViewMode::Raw;

                if is_empty {
                    Either::Left(view! {
                        <div class="welcome">
                            <h1>"Welcome to Markdown Viewer"</h1>
                            <p>"Click 'Open File' to view a markdown document"</p>
                            <p>"Press Ctrl+R to toggle between raw and rendered views"</p>
                        </div>
                    })
                } else if is_raw {
                    Either::Right(Either::Left(view! {
                        <div class="raw-view">
                            {current_content}
                        </div>
                    }))
                } else {
                    let html = markdown_to_html(&current_content);
                    Either::Right(Either::Right(view! {
                        <div class="markdown-body" inner_html=html></div>
                    }))
                }
            }}
        </div>
    }
}
