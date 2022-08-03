use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterUserRequest {
    #[validate]
    pub user: RegisterUserDto,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginUserRequest {
    #[validate]
    pub user: LoginUserDto,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserRequest {
    pub user: UpdateUserDto,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterUserDto {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginUserDto {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl RegisterUserDto {
    pub fn new_stub() -> Self {
        Self {
            username: Some(String::from("stub username")),
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}

impl LoginUserDto {
    pub fn new_stub() -> Self {
        Self {
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}
