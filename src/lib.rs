use std::{net::TcpListener, sync::Mutex};
use actix_web::{get, web, post, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio::runtime::Runtime;

mod util;
mod refresh_tokens;

use crate::refresh_tokens::refresh_tokens;


use crate::util::lmdb::{token_details::store_token_details, token::{store_token, get_token as get_token_lmdb}};



#[derive(Deserialize)]
struct AuthRequest {
    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
}


pub struct TokenManager {
    client_id: String, 
    client_secret: String,
    listener: TcpListener
}


#[derive(Clone)]
struct ClientData {
    client_id: String,
    client_secret: String,
    listener: String
}

lazy_static::lazy_static! {
    pub static ref SERVER_URL: Mutex<String> = Mutex::new("".to_string());
    pub static ref TOKEN_LOCK: Mutex<()> = Mutex::new(());
}

impl TokenManager {
    pub fn new(client_id: String, client_secret: String, listener: TcpListener) -> Self {
        {
            let mut server_url = SERVER_URL.lock().unwrap();
            *server_url = listener.local_addr().unwrap().to_string();

            refresh_tokens();
          
        }

        Self {
            client_id,
            client_secret, 
            listener
        }
    }

    pub async fn start_server(self) {
        println!("starting token manager.. ");

        self.start_actix_server()
    }

    pub fn get_token() -> String {

        let _lock = TOKEN_LOCK.lock().unwrap();


        let token = get_token_lmdb();
        token
    }

    fn start_actix_server(self) {
        std::thread::spawn(move || {
            let system = actix_rt::System::new();

            Runtime::new().unwrap().block_on(async {

                let data = ClientData {
                    client_id: self.client_id.clone(),
                    client_secret: self.client_secret.clone(),
                    listener: format!("http://{}/callback", self.listener.local_addr().expect("Failed to get local address").to_string().clone())
                };

                let srv = HttpServer::new(move || {
                    App::new()
                        .app_data(web::Data::new(data.clone()))
                        .route("/hello_world", web::get().to(greet))
                        .service(login)
                        .service(callback)
                        .service(refresh_token)
                })
                .listen(self.listener)
                .unwrap()
                .run();

                let _ = srv.await;
                
            });

            let _ = system.run();
        });
    }

    
}

async fn greet(data: web::Data<ClientData>) -> impl Responder {

    println!("client_id: {}\nclient_secret: {}\ncallback_url: {}", data.client_id, data.client_secret, data.listener);

    HttpResponse::Ok().body("Hello, world!")
}


#[get("/login")]
async fn login(client_data: web::Data<ClientData>) -> HttpResponse {
    let client_id = client_data.client_id.clone();
    let redirect_uri = client_data.listener.clone();

    let auth_url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope=user-read-private%20user-read-email",
        client_id, redirect_uri
    );
    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[get("/callback")]
async fn callback(client_data: web::Data<ClientData>, query: web::Query<AuthRequest>) -> HttpResponse {
    let client_id = client_data.client_id.clone();
    let redirect_uri = client_data.listener.clone();
    let client_secret = client_data.client_secret.clone();

    let params = [
        ("grant_type", "authorization_code"),
        ("code", query.code.as_str()),
        ("redirect_uri", redirect_uri.as_str()),
    ];

    let client = Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await
        .unwrap();

    let mut token_response: TokenResponse = response.json().await.unwrap();

    let current_timestamp = Utc::now();

    let expiration_timestamp = (current_timestamp + Duration::seconds(token_response.expires_in as i64 - 450 /*calculate -450 to prevent interupts*/)).timestamp();

    token_response.expires_in = expiration_timestamp as u64;

    let token_response_value: serde_json::Value = serde_json::to_value(&token_response).unwrap();

    let token_response_string: String = serde_json::to_string(&token_response_value).unwrap();

    println!("{:?}", &token_response_string);

    store_token_details(&token_response_string);
    store_token(&token_response.access_token);



    HttpResponse::Ok().json(token_response)

}

#[derive(Serialize,Deserialize)]
struct RefreshTokenRequest  {
    refresh_token: String
}

#[post("/refreshToken")]
async fn refresh_token(client_data: web::Data<ClientData>, req_body: String) -> HttpResponse {
    let client_id = client_data.client_id.clone();
    let client_secret = client_data.client_secret.clone();

    let refresh_token: RefreshTokenRequest = match serde_json::from_str(&req_body) {
        Ok(data) => data,
        Err(err) => {
            return HttpResponse::BadRequest().json(
                json!(
                    {
                        "error": err.to_string()
                    }
                )
            )
        }
    };

    println!("req body: {:?}", req_body);

    let client = Client::new();
    let res = client.post("https://accounts.spotify.com/api/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh_token.refresh_token),
        ])
        .send()
        .await;

    match res {
        Ok(response) => {
            let token_response: serde_json::Value = response.json().await.unwrap();

            return HttpResponse::Ok().json(token_response)
        }
        Err(e) => {
           
           return HttpResponse::BadRequest().json(json!({"error": e.to_string()}))

        }
    }


    
}

