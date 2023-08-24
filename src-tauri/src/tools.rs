
pub mod tasks {

    use std::path::Path;
    use regex::Regex;
    use serde_derive::{Deserialize, Serialize};
    use super::file_works;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Task {

        pub id: i32,
        pub text: String,
        pub is_done: bool
    }

    #[tauri::command]
    pub fn parse_file_tasks(path: &str) -> String {

        let text = file_works::get_text_from_file(Path::new(path));

        let mut result_arr: Vec<Task> = Vec::new();
        // если файл пустой возвращаем пустое значение
        if text.chars().count() == 0 {
            return "[]".to_string();
        }
        
        let re = Regex::new(r"-\s\[[\s|\w]\]\s*.*").unwrap();
        // разбиваем на каждую строку
        let mut i: i32 = 0;
        for elem in text.split("\n") {
            // применяем регулярное выражение для строки
            match re.captures(elem) {
                Some(caps) => {
                    // записываем результат увеличиваем индекс
                    result_arr.push(get_task_from_md_format(caps[0].to_string(), i));
                    i += 1;
                },
                _ => continue
            };
        }
        
        // результат сериализуем в json формат и отправляем на "клиент"
        let json = serde_json::to_string(&result_arr).unwrap();

        return json;
    }

    fn get_task_from_md_format(str: String, index: i32) -> Task {
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

}

pub mod file_works {

    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::*;

    pub fn get_text_from_file(path: &Path) -> String {
    
        // Откроем "путь" в режиме "только чтение". Возвращается `io::Result<File>`
        let mut file = match File::open(path) {
    
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

    

}

pub mod settings {
    
    use lazy_static;
    use tauri::api::path::{self, document_dir};
    use serde_derive::{Deserialize, Serialize};
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Write;
    use super::file_works;


    #[derive(Debug, Deserialize, Serialize)]
    pub struct Options {
        pub user_name: String,
        pub password: String,
        pub host: String,
        pub db_name: String
    }

    lazy_static::lazy_static! {
        static ref USER_OPTIONS: Options = Options { 
            user_name: "".to_string(),
            password: "".to_string(),
            host: "".to_string(),
            db_name: "".to_string()
        };
    }

    pub fn load_settings() {
        
        let mut true_path: PathBuf = document_dir().unwrap();

        true_path.push("ltTaskMessage");

        match true_path.exists() {
            false => {
                std::fs::create_dir(true_path.as_path()).expect("Ошибка создания папки");    
                true_path.push("settings.json");

                let mut file: File = File::create(true_path.as_path()).expect("Ошибка создания файла настроек");
                
                let opt = Options {user_name: "".to_string(), password: "".to_string(), host: "".to_string(), db_name: "".to_string()};
                
                writeln!(&mut file, "{}", serde_json::to_string(&opt).unwrap()).unwrap();

            },
            true => {}
        }
        

    }
}