// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;
mod db;

struct User {
    id: i32,
    name: String,
    password: String
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tools::tasks::parse_file_tasks, console_writeln, db::db::get_all_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// вывод в консоль
#[tauri::command]
fn console_writeln(text: String) {
    
    println!("Console log: {}", text);

}