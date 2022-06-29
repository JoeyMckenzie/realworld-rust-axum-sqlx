use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}
