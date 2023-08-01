// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;


extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;



// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_file_tasks, console_writeln])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// вывод в консоль
#[tauri::command]
fn console_writeln(text: String) {
    
    println!("Console log: {}", text);

}

#[derive(Debug, Deserialize, Serialize)]
struct Task {

    id: u16,
    text: String,
    is_done: bool
}

#[tauri::command]
fn parse_file_tasks(path: &str) -> String {

    let text = get_text_from_file(path);

    let mut result_arr: Vec<Task> = Vec::new();
    // если файл пустой возвращаем пустое значение
    if text.chars().count() == 0 {
        return "[]".to_string();
    }
    
    let re = Regex::new(r"-\s\[[\s|\w]\]\s*.*").unwrap();
    // разбиваем на каждую строку
    let mut i: u16 = 0;
    for elem in text.split("\n") {
        // применяем регулярное выражение для строки
        match re.captures(elem) {
            Some(caps) => {
                // записываем результат увеличиваем индекс
                result_arr.push(get_task_from_string(caps[0].to_string(), i));
                i += 1;
            },
            _ => continue
        };
    }
    
    // результат сериализуем в json формат и отправляем на "клиент"
    let json = serde_json::to_string(&result_arr).unwrap();

    return json;
}

fn get_task_from_string(str: String, index: u16) -> Task {
    // создаем объект задачи и присваиваем ему индекс
    let mut result_task: Task = Task { id: index, text: "".to_string(), is_done: true };

    // заменяем шаблон задачи и получаем сам текст задачи
    let re_get_text = Regex::new(r"-\s\[[\s|A-Za-zА-Яа-я]\]").unwrap();
    result_task.text = re_get_text.replace_all(&str, "").to_string();

    // считываем пустую задачу если пуста то не выполнена
    let re_check_is_done = Regex::new(r"-\s\[\s\]").unwrap();
    if re_check_is_done.is_match(&str) {
        result_task.is_done = false;
    }

    return result_task;
}

fn get_text_from_file(path: &str) -> String {
    
    let path = Path::new(path);
    // let display = path.display();

    // Откроем "путь" в режиме "только чтение". Возвращается `io::Result<File>`
    let mut file = match File::open(&path) {

        Err(why) => {
            println!("Ошибка: {}", why);
            return "".to_string()
        },
        Ok(file) => file,

    };

    // Читаем содержимое файла в строку. Метод возвращает `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            println!("Ошибка: {}", why);
            return "".to_string()
        },
        Ok(_) => return s,
    };
    

}