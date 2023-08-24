pub mod db {

    use lazy_static::lazy_static;
    //use serde_json::Result;
    use serde_derive::{Deserialize, Serialize};
    use std::{sync::{Arc, Mutex}, fmt::Error};
    //use postgres::{Client, NoTls, Error, Row};
    use tokio_postgres::{Client, NoTls, Row, error};
    use tauri::Result;
    use super::*;
    use crate::tools;

    pub struct Db {
        pub client: Client,
    }

    impl Db {

        pub async fn init(&self) {

            let current_db = self.get_list_tabels().await;
            
            if current_db.len() == 2 {
                return;
            }

            let create_users = "
            CREATE TABLE users
            (
                user_id SERIAL CONSTRAINT user_id PRIMARY KEY NOT NULL,
                user_name varchar(150) UNIQUE CHECK(user_name != ''),
                user_password varchar(150) CHECK(user_password != '')
            );"; 

            let create_tasks = "CREATE TABLE tasks
            (
                task_id SERIAL CONSTRAINT task_id PRIMARY KEY NOT NULL,
                task_text text CHECK(task_text != ''),
                is_done bool DEFAULT FALSE,
                author_id integer REFERENCES users (user_id)
            );";

            let add_admin = "INSERT INTO users (user_name, user_password)
            VALUES 
            (
                'admin',
                '123456'
            );";

            let add_task_for_admin = "INSERT INTO tasks (task_text, is_done, author_id)
            VALUES
            (
                'Check app for bugs',
                false,
                1
            );
            ";

            self.client.query(create_users, &[]).await.unwrap();
            self.client.query(create_tasks, &[]).await.unwrap();
            self.client.query(add_admin, &[]).await.unwrap();
            self.client.query(add_task_for_admin, &[]).await.unwrap();

        }

        pub async fn connect() -> Result<Db> {
            let (client, connection) = tokio_postgres::connect(
                "host= user= dbname= password=",
                NoTls,
            )
            .await
            .unwrap();

            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            Ok(Db { client })
        }

        async fn get_list_tabels(&self) -> Vec<Row> {
            
            let query = "
            SELECT tablename
            FROM pg_tables
            WHERE
                tablename = 'users' OR tablename = 'tasks'";
            let rows = self.client.query(query, &[]).await.unwrap();

            return rows;
            
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct User {
        id: i32,
        name: String,
        password: String
    }

    pub async fn db_init() {

        let db = Db::connect().await.expect("failed");
        db.init().await;

    }

}
        
    
    
   
