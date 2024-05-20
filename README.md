# spotify-api-token-manager
Manage your access tokens easily

## About this project
This project is currently in development state and is not meant for production purposes, use this **on your own risk**.
The development of this library depends on serveral other factors, this is one small core library of a larger project

## How to use this library

### prerequisites 
setup in the [spotify dashboard](https://developer.spotify.com/dashboard) your application, there are several tutorials online how to do that

what matters for us specifically are a few things

- client id
- client secret
- redirect url

please note that the redirect url should always end with `/callback`

here is a quick example usage

```rs
use spotify_token_manager::TokenManager;
use std::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let client_id = "05fd6ff8a5d84f399b5491410b9b22e5";
    let client_secret = "32b3b73d7ac4425bbf60484a5deab9f5";

    let init = TokenManager::new(client_id.to_string(), client_secret.to_string(), listener);

    init.start_server().await;

    let result = TokenManager::get_token();

    println!("result: {}", result);
}
```
note that the url of the listener always needs to be the same url you defined earlier in the spotify developer dashboard.
    
