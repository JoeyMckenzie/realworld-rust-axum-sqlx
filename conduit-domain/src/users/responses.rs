use crate::users::models::UserDto;

#[derive(serde::Serialize, Debug)]
pub struct UserAuthenicationResponse {
    pub user: UserDto,
}

impl UserAuthenicationResponse {
    pub fn new(username: String, email: String, bio: String, image: String, token: String) -> Self {
        UserAuthenicationResponse {
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
