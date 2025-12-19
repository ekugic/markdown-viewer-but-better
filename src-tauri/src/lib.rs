mod commands;

use std::sync::Mutex;

// Store the initial file path from CLI args
pub struct AppState {
    pub initial_file: Mutex<Option<String>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Get CLI arguments
    let args: Vec<String> = std::env::args().collect();
    let initial_file = if args.len() > 1 {
        // Just use the path as provided - let read_file_content handle errors
        let file_arg = args[1].clone();
        println!("Attempting to open file: {}", file_arg);
        Some(file_arg)
    } else {
        None
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            initial_file: Mutex::new(initial_file),
        })
        .invoke_handler(tauri::generate_handler![
            commands::select_markdown_file,
            commands::read_file_content,
            commands::get_initial_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
