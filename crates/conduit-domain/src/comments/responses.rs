use serde::{Deserialize, Serialize};

use crate::comments::CommentDto;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CommentResponse {
    pub comment: CommentDto,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CommentsResponse {
    pub comments: Vec<CommentDto>,
}
