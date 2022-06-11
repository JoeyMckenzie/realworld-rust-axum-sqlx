use conduit_core::errors::ConduitResult;
use conduit_core::users::service::DynUsersService;
use conduit_domain::users::requests::RegisterUserDto;

pub struct ConduitSeedService {
    users_service: DynUsersService,
}

impl ConduitSeedService {
    pub fn new(users_service: DynUsersService) -> Self {
        Self { users_service }
    }

    pub async fn seed(&self) -> ConduitResult<()> {
        // seed some users
        self.users_service
            .register_user(RegisterUserDto {
                username: Some(String::from("testuser1")),
                email: Some(String::from("testuser1@gmail.com")),
                password: Some(String::from("testuser1")),
            })
            .await?;

        self.users_service
            .register_user(RegisterUserDto {
                username: Some(String::from("testuser2")),
                email: Some(String::from("testuser2@gmail.com")),
                password: Some(String::from("testuser2")),
            })
            .await?;

        self.users_service
            .register_user(RegisterUserDto {
                username: Some(String::from("testuser3")),
                email: Some(String::from("testuser3@gmail.com")),
                password: Some(String::from("testuser3")),
            })
            .await?;

        Ok(())
    }
}
