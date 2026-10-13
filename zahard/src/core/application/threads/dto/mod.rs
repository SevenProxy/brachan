use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateThreadInput {
    pub board_id: i16,
    pub subject: Option<String>,
}
