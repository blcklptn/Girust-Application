#[path = "api/token.rs"] mod token;

mod models;

use reqwest::Client;

async fn autheficate() {
    let client = Client::new();
    let credentials = models::Credentials{
                                  login: "blcklptn".to_string(),
                                  password: "password".to_string()
                                 };
    token::get_token(credentials, client).await;
}


#[tokio::main]
async fn main() {
    autheficate().await;
}
