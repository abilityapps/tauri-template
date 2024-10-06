use std::process::Child;

use tauri::async_runtime::spawn;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

async fn start_pocketbase_server(app: AppHandle) -> Result<(), ()> {
    let sidecar_command = app.shell().sidecar("pocketbase").unwrap().args(["serve"]);
    let (mut _rx, mut _child) = sidecar_command.spawn().unwrap();
    println!("Pocketbase server started");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            spawn(start_pocketbase_server(app.handle().clone()));
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                println!("Pocketbase server stopped");
            }
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                println!("Pocketbase server stopped");
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
