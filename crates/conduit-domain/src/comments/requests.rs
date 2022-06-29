use serde::Deserialize;
use validator::Validate;

use crate::comments::CreateCommentDto;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate]
    pub comment: CreateCommentDto,
}
