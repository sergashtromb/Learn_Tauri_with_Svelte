// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tokio;

mod tools;
mod db;
mod commands;

#[tokio:: main]
async fn main() {

    tools::settings::load_global_settings();

    if tools::settings::GLOBAL_OPTIONS.lock().unwrap().have_db {
        db::db::db_init().await;
    }
    

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tools::tasks::parse_file_tasks,
            console_writeln,
            commands::com::check_user,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// вывод в консоль
#[tauri::command]
fn console_writeln(text: String) {
    
    println!("Console log: {}", text);

}