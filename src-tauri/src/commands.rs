pub mod com {

    use tokio::runtime::Handle;
    use serde_derive::{Deserialize, Serialize};
    use std::result::{Result};
    use crate::db::db;
    use crate::tools::tasks;
    use crate::tools::settings;
    use crate::tools::operations;

    #[tauri::command]
    pub async fn check_user_by_all(user_name: String, user_password: String) -> Result<String, String> {

        if settings::GLOBAL_OPTIONS.lock().await.have_db {

            let conn = db::Db::connect().await;

            match conn {
                Ok(data_base) => {

                    let new_user = data_base.get_user_by_all(user_name, user_password).await;
                    let result = serde_json::to_string(&new_user).unwrap();
        
                    return Ok(result);
                }
                Err(why) => {
                    return Err(why.to_string());
                }
            }
            
        }
        
        Ok("".to_string())
    }

    #[tauri::command]
    pub async fn check_user_by_name(user_name: String) -> Result<String, String> {

        if settings::GLOBAL_OPTIONS.lock().await.have_db {

            let conn = db::Db::connect().await;

            match conn {
                Ok(data_base) => {

                    let new_user = data_base.get_user_by_name(user_name).await;
                    let result = serde_json::to_string(&new_user).unwrap();
                    
                    return Ok(result);
                }
                Err(why) => {
                    return Err(why.to_string());
                }
            }
            
        }
        
        Ok("".to_string())
    }

    #[tauri::command]
    pub async fn registration_user(user_name: String, user_password: String) -> Result<String, String> {

        if settings::GLOBAL_OPTIONS.lock().await.have_db {
            
            let conn = db::Db::connect().await;

            match conn {
                Ok(data_base) => {

                    let new_user = tasks::User {
                        id: data_base.create_user(user_name.clone(), user_password.clone()).await,
                        user_name: user_name,
                        user_password: user_password,
                    };
        
                    let result = serde_json::to_string(&new_user).unwrap();
        
                    return Ok(result);

                }
                Err(why) => {

                    let oper = operations::Operation {
                        type_operation: operations::TypeOperation::CreateUser,
                        task: tasks::Task { id: -1, text: "".to_string(), is_done: false },
                        user: db::User { id: -1, name: user_name, password: user_password }
                    };
                    operations::add_operation(oper).await;

                    return Err(why.to_string());
                }
            }

        }
        
        Ok("".to_string())
    }

    #[tauri::command]
    pub async fn get_tasks(user_id: i32) -> Result<String, String> {

        if settings::GLOBAL_OPTIONS.lock().await.have_db {

            let conn = db::Db::connect().await;

            match conn {
                Ok(data_base) => {
                    let tasks = data_base.get_tasks_by_user_id(user_id).await;
                    let result = serde_json::to_string(&tasks).unwrap();

                    return Ok(result);
                }
                Err(why) => {
                    return Err(why.to_string());
                }
            }

        }
        Ok("".to_string())
    }

}