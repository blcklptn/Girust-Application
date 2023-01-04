use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Credentials{
    pub login: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct Tokens{
    pub token: String,
    pub refresh_token: String
}