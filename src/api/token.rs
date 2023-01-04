use reqwest::Client;
use crate::models::{Tokens, Credentials};

pub async fn get_token(creds: Credentials, client: Client){
    let response = client.post("http://127.0.0.1:5000/api/auth")
                                        .json(&creds)
                                        .send()
                                        .await
                                        .expect("Failed")
                                        .text()
                                        .await
                                        .expect("Failed");
    let tokens: Tokens = serde_json::from_str(&response).unwrap();
    println!("{} - {}", tokens.token, tokens.refresh_token);
}