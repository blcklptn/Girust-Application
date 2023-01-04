
pub async fn concate(method: &str) -> String{
    let mut owner_string: String = dotenv!("HOST").to_owned();
    owner_string.push_str(method);
    return owner_string
}