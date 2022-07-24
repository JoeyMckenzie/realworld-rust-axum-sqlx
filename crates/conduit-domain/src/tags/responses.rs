use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}
