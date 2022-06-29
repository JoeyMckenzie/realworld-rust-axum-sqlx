use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Default)]
pub struct RegisterUserRequest {
    #[validate]
    pub user: RegisterUserDto,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginUserRequest {
    #[validate]
    pub user: LoginUserDto,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub user: UpdateUserDto,
}

#[derive(Clone, Deserialize, Debug, Validate, Default)]
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

#[derive(Deserialize, Debug)]
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