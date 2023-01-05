#[path = "../middlewares/concatenate.rs"] mod concatenate;
use reqwest::Client;
use crate::models::Credentials;


pub async fn auth(creds: Credentials, client: Client) -> String {
    let url = concatenate::concate("/api/auth").await;    
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

pub async fn refresh(refresh_token: String, client: Client) {
    let url = concatenate::concate("/api/refresh").await;
    println!("{} - {}", url, refresh_token);

}