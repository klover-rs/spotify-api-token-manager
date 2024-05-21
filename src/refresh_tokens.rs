use crate::util::lmdb::{token::store_token, token_details::{get_token_details, store_token_details}};
use reqwest::Client;
use serde_json::json;
use tokio::task;
use tokio::time::{sleep, Duration};
use user_idle::UserIdle;
use chrono::{Utc, Duration as CDuration};

use std::sync::Once;


use crate::{SERVER_URL, TOKEN_LOCK};

static INIT: Once = Once::new();

pub fn refresh_tokens() {
    task::spawn(async move {
        loop {
            let token_details = get_token_details();
            

            if !token_details.is_empty() {
                let token_details_value: serde_json::Value = serde_json::from_str(&token_details).unwrap();

                let expiration_timestamp = token_details_value.get("expires_in").unwrap().as_i64().unwrap();

                let current_timestamp = Utc::now().timestamp();

                if current_timestamp > expiration_timestamp {
                    let refresh_token = token_details_value.get("refresh_token").unwrap().as_str().unwrap();

                    let new_data = refresh_access_token(&refresh_token).await.unwrap();

                    println!("new token: {:?}", new_data);

                    INIT.call_once(|| {
                        std::mem::drop(TOKEN_LOCK.lock().unwrap());
                    });
                    
                    println!("unlocked resource");

                    let current_timestamp = Utc::now();

                    let expiration_timestamp = (current_timestamp + CDuration::seconds(3600 - 450 /*calculate -450 to prevent interupts*/)).timestamp();

                    let new_access_token = new_data.get("access_token").unwrap().as_str().unwrap();

                    let new_token_details = json!({
                        "access_token": new_access_token,
                        "expires_in": expiration_timestamp,
                        "token_type": new_data.get("access_token").unwrap().as_str().unwrap(),
                        "refresh_token": refresh_token
                    }).to_string();

                    store_token_details(&new_token_details);
                    store_token(&new_access_token);
                } else {
                    INIT.call_once(|| {
                        std::mem::drop(TOKEN_LOCK.lock().unwrap());
                    });
                    println!("unlocked resource");
                }
    
            }

            const IDLE_THRESHOLD_SECS: u64 = 120;

            let system_idle = UserIdle::get_time().unwrap().as_seconds();

            let sleep_time_secs = if system_idle > IDLE_THRESHOLD_SECS {
                30
                
            } else {
                10
            };

            sleep(Duration::from_secs(sleep_time_secs)).await;
        }
    });
}

async fn refresh_access_token(refresh_token: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = Client::new();

    let server_url = SERVER_URL.lock().unwrap().to_string();

    println!("SERVER URL: {}", server_url);

    let response = client
        .post(format!("http://{}/refreshToken", server_url))
        .body(json!({"refresh_token": refresh_token}).to_string())
        .send()
        .await?;

    let json_response: serde_json::Value = response.json().await?;

    Ok(json_response)

}


