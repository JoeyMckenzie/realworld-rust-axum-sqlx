use crate::users::UserDto;

#[derive(serde::Serialize, Debug)]
pub struct UserAuthenicationResponse {
    pub user: UserDto,
}

impl UserAuthenicationResponse {
    pub fn new(id: i64, username: String, email: String, bio: String, image: String, token: String) -> Self {
        UserAuthenicationResponse {
            user: UserDto {
                id,
                username,
                email,
                bio,
                image,
                token,
            },
        }
    }
}
