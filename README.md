# spotify-api-token-manager
Manage your access tokens easily

## About this project
This project is currently in development state and is not meant for production purposes, use this **on your own risk**.
The development of this library depends on serveral other factors, this is one small core library of a larger project

## How to use this library

### how to add the library to your cargo

(expecting your directory tree looks like this)
```
├───libtester
│   └───src
├───spotify-api-token-manager
│   ├───src
│   │   └───util
│   │       └───lmdb
│   └───tests
└───spotify_api_data
    └───src
```
add this to your Cargo.toml 
```toml
[dependencies]
spotify_token_manager = {path = "../spotify-api-token-manager"}
```

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
use tokio::signal;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let client_id = "05fd6ff8a5d84f399b5491410b9b22e5";
    let client_secret = "32b3b73d7ac4425bbf60484a5deab9f5";

    let init = TokenManager::new(client_id.to_string(), client_secret.to_string(), listener);

    init.start_server().await;

    let result = TokenManager::get_token();

    println!("result: {}", result);

    signal::ctrl_c().await.expect("failed to listen for Ctrl+C"); //keep the the thread alive, if you dont keep the thread alive unexpected issue will occur. 
}
```
note that the url of the listener always needs to be the same url you defined earlier in the spotify developer dashboard.

## also important
please note that this library will create a folder in your home dir called "stm_data" which stands simply for spotify token manage data :D
so make sure that you dont already have a folder which is called like this
    
