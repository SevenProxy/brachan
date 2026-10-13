use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateBoardInput {
    pub slug: String,
    pub name: String,
    pub description: String,
}
