#[path = "../middlewares/concatenate.rs"] mod concatenate;
use reqwest::Client;
use crate::models::Credentials;


pub async fn get_token(creds: Credentials, client: Client) -> String {
    let url = concatenate::concate("/api/login").await;    
    println!("{}", url);
    let response = client.post(url)
                                        .json(&creds)
                                        .send()
                                        .await
                                        .expect("Failed")
                                        .text()
                                        .await
                                        .expect("Failed");
    return response;
}