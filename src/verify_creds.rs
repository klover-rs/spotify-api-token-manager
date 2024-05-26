use reqwest::Client;
use anyhow::Result;
use base64::prelude::*;

pub async fn verify_creds(client_id: &str, client_secret: &str) -> Result<bool> {
    
    let auth_header = BASE64_STANDARD.encode(format!("{}:{}", client_id, client_secret));

    let client = Client::new();

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {}", auth_header))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?;

    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }

    
}
