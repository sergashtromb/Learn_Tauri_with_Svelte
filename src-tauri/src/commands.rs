pub mod com {

    use tokio;
    use serde_derive::{Deserialize, Serialize};
    use crate::db::db;

    #[tauri::command]
    pub async fn check_user(user_name: String, user_password: String) -> String {

        let data_base = db::Db::connect().await.unwrap();
        let new_user = data_base.get_user_by_name(user_name, user_password).await;
        let result = serde_json::to_string(&new_user).unwrap();

        return result;
    }
}