#[derive(serde::Serialize, Debug)]
pub struct UserDto {
    pub username: String,
    pub email: String,
    pub bio: String,
    pub image: String,
    pub token: String,
}
