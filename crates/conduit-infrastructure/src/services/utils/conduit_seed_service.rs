use lazy_static::lazy_static;
use tracing::info;

use conduit_core::articles::service::DynArticlesService;
use conduit_core::comments::service::DynCommentsService;
use conduit_core::errors::ConduitResult;
use conduit_core::profiles::service::DynProfilesService;
use conduit_core::users::service::DynUsersService;
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto};
use conduit_domain::users::UserDto;

use crate::service_register::ServiceRegister;

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
    profiles_service: DynProfilesService,
    articles_service: DynArticlesService,
    comments_service: DynCommentsService,
}

impl ConduitSeedService {
    pub fn new(service_register: ServiceRegister) -> Self {
        Self {
            users_service: service_register.users_service,
            profiles_service: service_register.profiles_service,
            articles_service: service_register.articles_service,
            comments_service: service_register.comments_service,
        }
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

        info!("seeding users...");
        let created_user_1 = self
            .create_user(*TEST_USER_1_USERNAME, *TEST_USER_1_EMAIL, *TEST_USER_1_PASSWORD)
            .await?;

        let created_user_2 = self
            .create_user(*TEST_USER_2_USERNAME, *TEST_USER_2_EMAIL, *TEST_USER_2_PASSWORD)
            .await?;

        let created_user_3 = self
            .create_user(*TEST_USER_3_USERNAME, *TEST_USER_3_EMAIL, *TEST_USER_3_PASSWORD)
            .await?;

        info!("users created, seeding user follows...");
        // user 2 follows user 1
        self.profiles_service
            .add_user_follow(&created_user_1.username, created_user_2.id)
            .await?;

        // user 3 follows user 1
        self.profiles_service
            .add_user_follow(&created_user_1.username, created_user_3.id)
            .await?;

        // user 3 follows user 2
        self.profiles_service
            .add_user_follow(&created_user_2.username, created_user_3.id)
            .await?;

        info!("user follows created, seeding articles...");
        let article_1 = self
            .articles_service
            .create_article(
                created_user_1.id,
                String::from("testuser1 article 1"),
                String::from("testuser1 description 1"),
                String::from("testuser1 body 1"),
                vec![String::from("tag1"), String::from("tag2")],
            )
            .await?;

        self.articles_service
            .create_article(
                created_user_1.id,
                String::from("testuser1 article 2"),
                String::from("testuser1 description 2"),
                String::from("testuser1 body 2"),
                vec![String::from("tag2"), String::from("tag3")],
            )
            .await?;

        self.articles_service
            .create_article(
                created_user_2.id,
                String::from("testuser2 article 1"),
                String::from("testuser2 description 1"),
                String::from("testuser2 body 1"),
                vec![],
            )
            .await?;

        info!("articles created, seeding comments...");

        // user 2 comments on user 1's article 1
        self.comments_service
            .add_comment(
                created_user_2.id,
                article_1.slug.clone(),
                String::from("testuser2 comments on article 1"),
            )
            .await?;

        // user 2 comments on user 1's article 2
        self.comments_service
            .add_comment(
                created_user_2.id,
                article_1.slug,
                String::from("testuser2 comments on article 2"),
            )
            .await?;

        info!("seed ran successfully!");

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
