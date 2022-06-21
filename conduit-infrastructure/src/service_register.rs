use std::sync::Arc;

use tracing::info;

use conduit_core::articles::repository::DynArticlesRepository;
use conduit_core::articles::service::DynArticlesService;
use conduit_core::comments::repository::DynCommentsRepository;
use conduit_core::comments::service::DynCommentsService;
use conduit_core::config::AppConfig;
use conduit_core::profiles::repository::DynProfilesRepository;
use conduit_core::profiles::service::DynProfilesService;
use conduit_core::tags::repository::DynTagsRepository;
use conduit_core::tags::service::DynTagsService;
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::users::service::DynUsersService;
use conduit_core::utils::security_service::DynSecurityService;
use conduit_core::utils::token_service::DynTokenService;

use crate::connection_pool::ConduitConnectionPool;
use crate::repositories::articles_repository::PostgresArticlesRepository;
use crate::repositories::comments_repository::PostgresCommentsRepository;
use crate::repositories::profiles_repository::PostgresProfilesRepository;
use crate::repositories::tags_repository::PostgresTagsRepository;
use crate::repositories::users_repository::PostgresUsersRepository;
use crate::services::features::articles_service::ConduitArticlesService;
use crate::services::features::comments_service::ConduitCommentsService;
use crate::services::features::profiles_service::ConduitProfilesService;
use crate::services::features::tags_service::ConduitTagsService;
use crate::services::features::users_service::ConduitUsersService;
use crate::services::utils::argon_security_service::ArgonSecurityService;
use crate::services::utils::jwt_service::JwtService;

#[derive(Clone)]
pub struct ServiceRegister {
    pub users_service: DynUsersService,
    pub token_service: DynTokenService,
    pub profiles_service: DynProfilesService,
    pub articles_service: DynArticlesService,
    pub comments_service: DynCommentsService,
    pub tags_service: DynTagsService,
}

/// A simple service container responsible for managing the various services our API endpoints will pull from through axum extensions.
impl ServiceRegister {
    pub fn new(pool: ConduitConnectionPool, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");
        let security_service = Arc::new(ArgonSecurityService::new(config.clone())) as DynSecurityService;
        let token_service = Arc::new(JwtService::new(config)) as DynTokenService;

        info!("utility services initialized, building feature services...");
        let users_repository = Arc::new(PostgresUsersRepository::new(pool.clone())) as DynUsersRepository;
        let users_service = Arc::new(ConduitUsersService::new(
            users_repository.clone(),
            security_service,
            token_service.clone(),
        )) as DynUsersService;

        let profiles_repository = Arc::new(PostgresProfilesRepository::new(pool.clone())) as DynProfilesRepository;
        let profiles_service = Arc::new(ConduitProfilesService::new(
            users_repository.clone(),
            profiles_repository,
        )) as DynProfilesService;

        let tags_repository = Arc::new(PostgresTagsRepository::new(pool.clone())) as DynTagsRepository;
        let tags_service = Arc::new(ConduitTagsService::new(tags_repository.clone())) as DynTagsService;

        let articles_repository = Arc::new(PostgresArticlesRepository::new(pool.clone())) as DynArticlesRepository;
        let articles_service = Arc::new(ConduitArticlesService::new(
            articles_repository.clone(),
            tags_repository,
        )) as DynArticlesService;

        let comments_repository = Arc::new(PostgresCommentsRepository::new(pool)) as DynCommentsRepository;
        let comments_service =
            Arc::new(ConduitCommentsService::new(comments_repository, articles_repository)) as DynCommentsService;

        info!("feature services successfully initialized!");

        ServiceRegister {
            users_service,
            token_service,
            profiles_service,
            articles_service,
            comments_service,
            tags_service,
        }
    }
}
