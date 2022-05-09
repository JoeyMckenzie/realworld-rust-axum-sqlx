use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RegisterUserResponse {
    pub user: UserDto,
}

impl RegisterUserResponse {
    pub fn new(username: String, email: String) -> Self {
        let user = UserDto { username, email };

        RegisterUserResponse { user }
    }
}
