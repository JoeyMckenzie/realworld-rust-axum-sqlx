use serde::Serialize;

use crate::comments::CommentDto;

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub comment: CommentDto,
}

#[derive(Debug, Serialize)]
pub struct CommentsResponse {
    pub comments: Vec<CommentDto>,
}
