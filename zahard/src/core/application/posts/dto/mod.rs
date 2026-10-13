use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePostInput {
    pub thread_id: i64,
    pub local_number: i32,
    pub is_op: bool,
    pub body: String,
    pub image_id: Option<i64>,
    pub poster_id: Option<String>,
    pub ip_hash: Vec<u8>,
    pub is_sage: bool,
}
