use std::fs;
use tauri::State;

#[tauri::command]
pub fn get_initial_file(state: State<crate::AppState>) -> Result<Option<String>, String> {
    state.initial_file
        .lock()
        .map(|guard| guard.clone())
        .map_err(|e| format!("Failed to get initial file: {}", e))
}

#[tauri::command]
pub async fn select_markdown_file(app: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;

    // Create a channel to receive the result
    let (tx, rx) = std::sync::mpsc::channel();

    // Use the non-blocking pick_file with a callback
    app.dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown", "mdown", "mkd"])
        .add_filter("Text", &["txt"])
        .add_filter("All Files", &["*"])
        .set_title("Select a Markdown File")
        .pick_file(move |file_path| {
            let _ = tx.send(file_path);
        });

    // Wait for the callback result
    match rx.recv() {
        Ok(Some(path)) => Ok(path.to_string()),
        Ok(None) => Err("No file selected".to_string()),
        Err(_) => Err("Dialog was closed or failed".to_string()),
    }
}

#[tauri::command]
pub async fn read_file_content(path: String) -> Result<String, String> {
    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file {}: {}", path, e))
}
