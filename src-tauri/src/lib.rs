use sqlx::PgPool;
// use sqlx::SqlitePool; // Uncomment this line and remove PgPool import if you're using SQLite
use std::env;
use tokio::runtime::Runtime;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();

    let runtime = Runtime::new().expect("Failed to create runtime");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool: PgPool = runtime.block_on(async {
        PgPool::connect(&database_url)
            .await
            .expect("Failed to create PostgreSQL pool")
    });
    // let pool: SqlitePool = runtime.block_on(async { // Uncomment this, and remove PgPool if you're using SQLite
    //     SqlitePool::connect(&database_url)
    //         .await
    //         .expect("Failed to create SQLite pool")
    // });

    tauri::Builder::default()
        .manage(pool)
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
