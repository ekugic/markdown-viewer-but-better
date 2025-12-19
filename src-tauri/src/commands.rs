use std::fs;
use std::sync::mpsc;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn select_markdown_file(app: tauri::AppHandle) -> Result<String, String> {
    let (tx, rx) = mpsc::channel();

    tauri_plugin_dialog::FileDialogBuilder::new(app.dialog().clone())
        .add_filter("Markdown", &["md", "markdown", "mdown", "mkd"])
        .add_filter("Text", &["txt"])
        .add_filter("All Files", &["*"])
        .set_title("Select a Markdown File")
        .pick_file(move |file_path| {
            let _ = tx.send(file_path.map(|p| p.to_string()));
        });

    // Wait for the dialog result using the channel
    match rx.recv() {
        Ok(Some(path)) => Ok(path),
        Ok(None) => Err("No file selected".to_string()),
        Err(_) => Err("Failed to receive file selection".to_string()),
    }
}

#[tauri::command]
pub async fn read_file_content(path: String) -> Result<String, String> {
    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file {}: {}", path, e))
}
