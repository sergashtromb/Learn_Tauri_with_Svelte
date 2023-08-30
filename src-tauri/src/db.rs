pub mod db {

    use serde_derive::{Deserialize, Serialize};
    use tokio_postgres::{Client, NoTls, Row};
    use tauri::Result;
    use crate::tools;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DataBaseConfig {

        pub user_name: String,
        pub password: String,
        pub host: String,
        pub db_name: String

    }
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

            let user_settings = tools::settings::GLOBAL_OPTIONS.lock().await;

            let (client, connection) = tokio_postgres::connect(
                format!("host={} user={} dbname={} password={}", 
                        user_settings.db_config.host,
                        user_settings.db_config.user_name,
                        user_settings.db_config.db_name,
                        user_settings.db_config.password
                    ).as_str(),
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

        pub async fn get_user_by_all(&self, user_name: String, user_password: String) -> tools::tasks::User {

            let query = "
                SELECT user_id, user_name, user_password
                FROM users
                WHERE
                    user_name = $1 AND user_password = $2
            ";
        
            let rows = self.client.query(query, &[&user_name, &user_password]).await.unwrap();

            if rows.len() == 0 {
                return tools::tasks::User {
                    id: -1,
                    user_name: "".to_string(),
                    user_password: "".to_string()
                };
            }

            return tools::tasks::User {
                id: rows[0].get::<_, i32>(0),
                user_name: rows[0].get::<_, String>(1),
                user_password: rows[0].get::<_, String>(2)
            };
        }

        pub async fn get_user_by_name(&self, user_name: String) -> tools::tasks::User {

            let query = "
                SELECT user_id, user_name, user_password
                FROM users
                WHERE
                    user_name = $1
            ";
        
            let rows = self.client.query(query, &[&user_name]).await.unwrap();

            if rows.len() == 0 {
                return tools::tasks::User {
                    id: -1,
                    user_name: "".to_string(),
                    user_password: "".to_string()
                };
            }

            return tools::tasks::User {
                id: rows[0].get::<_, i32>(0),
                user_name: rows[0].get::<_, String>(1),
                user_password: rows[0].get::<_, String>(2)
            };


        }

        pub async fn create_user(&self, user_name: String, user_password: String) -> i32 {

            let query ="
            INSERT INTO users (user_name, user_password)
            VALUES ($1, $2) RETURNING user_id
            ";

            let row = self.client.query(query, &[&user_name, &user_password]).await.unwrap();

            return row[0].get::<_, i32>(0);
        }

        pub async fn get_tasks_by_user_id(&self, user_id: i32) -> Vec<tools::tasks::Task> {

            let query = "
                SELECT task_id, task_text, is_done
                FROM tasks
                WHERE
                    author_id = $1
            ";

            let rows = self.client.query(query, &[&user_id]).await.unwrap();

            let mut result_vec: Vec<tools::tasks::Task> = vec![];

            for row in rows {
                result_vec.push(tools::tasks::Task {
                    id: row.get::<_, i32>(0),
                    text: row.get::<_, String>(0),
                    is_done: row.get::<_, bool>(2)
                });
            }

            return result_vec;
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
        
    
    
   
