#[path = "api/token.rs"] mod token;
mod models;
use models::Tokens;
use reqwest::Client;
use std::sync::RwLock;

#[macro_use]
extern crate dotenv_codegen;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref AUTHTOKEN: RwLock<String> = RwLock::new("None".to_string());
    static ref REFRESHTOKEN: RwLock<String> = RwLock::new("None".to_string());
}

async fn autheficate() {
    let client = Client::new();
    let credentials = models::Credentials{
                                  login: "blcklptn".to_string(),
                                  password: "password".to_string()
                                 };
    let tokens_serialized: String = token::auth(credentials, client).await;
    let tokens: Tokens = serde_json::from_str(tokens_serialized.as_str()).unwrap();
    
    {
        let mut static_write_access = AUTHTOKEN.write().unwrap();
        let mut static_refresh_access = REFRESHTOKEN.write().unwrap();
        *static_write_access = tokens.token.to_string();
        *static_refresh_access = tokens.refresh_token.to_string();
    }
}


#[tokio::main]
async fn main() {
    
    autheficate().await;
    
}
