use std::env;
use tauri::Manager;
use tokio::runtime::Runtime;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let runtime = Runtime::new().expect("Failed to create runtime");

    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle();
            let handle_clone = app_handle.clone();

            let database = runtime.block_on(async move {
                db::Database::new(handle_clone)
                    .await
                    .expect("Failed to initialize database")
            });

            app.manage(db::DatabaseState(database.pool));
            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
