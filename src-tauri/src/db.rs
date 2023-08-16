pub mod db {

    use lazy_static;
    use serde_json::Result;

    use std::sync::{Arc, Mutex};
    use postgres::{Client, NoTls, Error, Row};

    lazy_static::lazy_static! {
        static ref DATABASE_CONNECTION: Arc<Mutex<Client>> = {
            let client = Client::connect("", NoTls).expect("Failed to connect to database");
            Arc::new(Mutex::new(client))
        };
    }

    #[tauri::command]
    pub fn get_all_tasks() {

        let mut client = DATABASE_CONNECTION.lock().unwrap();
        let result =  match client.query("SELECT * FROM users", &[]) {
            Ok(result) => result,
            Err(why) => {
                panic!("Ошибка {}", why);
            },
        };
        for row in result {
            println!("{:?}", row);
        }
        
    }

}