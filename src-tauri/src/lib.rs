mod config;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    let mut screen_size = (0, 0);
    tauri::Builder::default()
        .setup(move |app| {
            let binding = app.primary_monitor().unwrap().unwrap();
            let screen = binding.size();
            screen_size = (screen.width, screen.height);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
