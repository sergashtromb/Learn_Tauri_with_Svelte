pub mod db {

    use lazy_static;
    //use serde_json::Result;
    use serde_derive::{Deserialize, Serialize};
    use std::sync::{Arc, Mutex};
    use postgres::{Client, NoTls, Error, Row};

    lazy_static::lazy_static! {
        static ref DATABASE_CONNECTION: Arc<Mutex<Client>> = {
            let client = Client::connect("", NoTls).expect("Failed to connect to database");
            Arc::new(Mutex::new(client))
        };
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct User {
        id: i32,
        name: String,
        password: String
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
            println!("{:?} - {:?} - {:?}", row.get::<_, i32>(0), row.get::<_, String>(1), row.get::<_, String>(2));
        }
        
    }

    #[tauri::command]
    pub fn get_user(user_name: String, user_password: String) -> String {

        let mut json_result : Vec<User> = Vec::new();
        let mut client = DATABASE_CONNECTION.lock().unwrap();

        match client.query(
            "SELECT user_id, user_name, user_password
            FROM users
            WHERE
                user_name = $1 AND user_password = $2", &[&user_name, &user_password]) {

            Ok(result) => {
                if result.len() > 0 {
                    json_result.push(User { id: result[0].get::<_, i32>(0), name: result[0].get::<_, String>(1), password: result[0].get::<_, String>(2) });
                }
                
            },
            Err(why) => {
                println!("Ошибка - {}", why);
            },
        };

        return serde_json::to_string(&json_result).unwrap();
        
    }

}