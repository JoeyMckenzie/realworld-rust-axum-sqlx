use lazy_static::lazy_static;
use tracing::info;

use conduit_core::errors::ConduitResult;
use conduit_core::users::service::DynUsersService;
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto};
use conduit_domain::users::UserDto;

lazy_static! {
    static ref TEST_USER_1_USERNAME: &'static str = "testuser1";
    static ref TEST_USER_1_EMAIL: &'static str = "testuser1@gmail.com";
    static ref TEST_USER_1_PASSWORD: &'static str = "testuser1";
    static ref TEST_USER_2_USERNAME: &'static str = "testuser2";
    static ref TEST_USER_2_EMAIL: &'static str = "testuser2@gmail.com";
    static ref TEST_USER_2_PASSWORD: &'static str = "testuser2";
    static ref TEST_USER_3_USERNAME: &'static str = "testuser3";
    static ref TEST_USER_3_EMAIL: &'static str = "testuser3@gmail.com";
    static ref TEST_USER_3_PASSWORD: &'static str = "testuser3";
}

pub struct ConduitSeedService {
    users_service: DynUsersService,
}

impl ConduitSeedService {
    pub fn new(users_service: DynUsersService) -> Self {
        Self { users_service }
    }

    pub async fn seed(&self) -> ConduitResult<()> {
        // assume that if we have an active user in the users table, data has been seeded
        let seed_data_exists = self
            .users_service
            .login_user(LoginUserDto {
                email: Some(String::from(*TEST_USER_1_EMAIL)),
                password: Some(String::from(*TEST_USER_1_PASSWORD)),
            })
            .await
            .is_ok();

        if seed_data_exists {
            info!("data has already been seeded, bypassing test data setup");
            return Ok(());
        }

        self.create_user(
            *TEST_USER_1_USERNAME,
            *TEST_USER_1_EMAIL,
            *TEST_USER_1_PASSWORD,
        )
        .await?;
        self.create_user(
            *TEST_USER_2_USERNAME,
            *TEST_USER_2_EMAIL,
            *TEST_USER_2_PASSWORD,
        )
        .await?;
        self.create_user(
            *TEST_USER_3_USERNAME,
            *TEST_USER_3_EMAIL,
            *TEST_USER_3_PASSWORD,
        )
        .await?;

        Ok(())
    }

    async fn create_user(
        &self,
        username: &'static str,
        email: &'static str,
        password: &'static str,
    ) -> ConduitResult<UserDto> {
        self.users_service
            .register_user(RegisterUserDto {
                username: Some(String::from(username)),
                email: Some(String::from(email)),
                password: Some(String::from(password)),
            })
            .await
    }
}
