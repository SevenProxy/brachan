use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
}
