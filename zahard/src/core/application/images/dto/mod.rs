use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateImageInput {
    pub disk_name: String,
    pub thumb_name: Option<String>,
    pub mime_type: String,
    pub size_bytes: i32,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub sha256: Vec<u8>,
}
