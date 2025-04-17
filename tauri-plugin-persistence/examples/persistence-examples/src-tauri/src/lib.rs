pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_persistence::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
