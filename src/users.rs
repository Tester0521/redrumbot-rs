use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use teloxide::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    telegram_id: i64,
    first_name: String,
}

pub async fn load_users(file_path: &str) -> Result<Vec<UserData>, Box<dyn std::error::Error>> {
    if let Ok(data) = fs::read_to_string(file_path).await {
        let users: Vec<UserData> = serde_json::from_str(&data)?;
        Ok(users)
    } else {
        Ok(Vec::new()) 
    }
}

pub async fn save_user(file_path: &str, users_data: &Arc<Mutex<Vec<UserData>>>, user_id: i64, first_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut users_data = users_data.lock().await;

    if let None = users_data.iter().find(|user| user.telegram_id == user_id) {
        users_data.push(UserData {
            telegram_id: user_id,
            first_name: first_name.to_string(),
        });

        let data = serde_json::to_string(&*users_data)?;
        fs::write(file_path, data).await?;
    } 

    Ok(())
}