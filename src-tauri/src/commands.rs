pub mod com {

    use tokio;
    use serde_derive::{Deserialize, Serialize};
    use crate::db::db;
    use crate::tools::tasks;

    #[tauri::command]
    pub async fn check_user_by_all(user_name: String, user_password: String) -> String {

        let data_base = db::Db::connect().await.unwrap();
        let new_user = data_base.get_user_by_all(user_name, user_password).await;
        let result = serde_json::to_string(&new_user).unwrap();
        
        return result;
    }

    #[tauri::command]
    pub async fn check_user_by_name(user_name: String) -> String {

        let data_base = db::Db::connect().await.unwrap();
        let new_user = data_base.get_user_by_name(user_name).await;
        let result = serde_json::to_string(&new_user).unwrap();
        
        return result;
    }

    #[tauri::command]
    pub async fn registration_user(user_name: String, user_password: String) -> String {

        let data_base = db::Db::connect().await.unwrap();
        let new_user = tasks::User {
            id: data_base.create_user(user_name.clone(), user_password.clone()).await,
            user_name: user_name,
            user_password: user_password,
        };

        let result = serde_json::to_string(&new_user).unwrap();

        return result;
    }
}