use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterUserRequest {
    #[validate]
    pub user: Option<RegisterUserDto>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginUserRequest {
    #[validate]
    pub user: Option<LoginUserDto>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterUserDto {
    #[validate(required)]
    pub username: Option<String>,
    #[validate(required, email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required)]
    pub password: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginUserDto {
    #[validate(required, email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required)]
    pub password: Option<String>,
}
