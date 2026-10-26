use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserApplication {
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}
