use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePostReferenceInput {
    pub post_id: i64,
    pub referenced_id: i64,
    pub thread_id: i64,
}