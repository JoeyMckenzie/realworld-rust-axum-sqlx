use crate::users::models::UserDto;

#[derive(serde::Serialize, Debug)]
pub struct RegisterUserResponse {
    pub user: UserDto,
}

impl RegisterUserResponse {
    pub fn new(username: String, email: String, bio: String, image: String, token: String) -> Self {
        RegisterUserResponse {
            user: UserDto {
                username,
                email,
                bio,
                image,
                token,
            },
        }
    }
}
