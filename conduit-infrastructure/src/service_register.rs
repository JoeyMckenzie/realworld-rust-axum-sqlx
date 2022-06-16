use std::sync::Arc;

use tracing::info;

use conduit_core::articles::repository::DynArticlesRepository;
use conduit_core::articles::service::DynArticlesService;
use conduit_core::config::AppConfig;
use conduit_core::errors::ConduitResult;
use conduit_core::profiles::repository::DynProfilesRepository;
use conduit_core::profiles::service::DynProfilesService;
use conduit_core::tags::repository::DynTagsRepository;
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::users::service::DynUsersService;
use conduit_core::utils::security_service::DynSecurityService;
use conduit_core::utils::token_service::DynTokenService;
use conduit_core::utils::unit_of_work::DynUnitOfWork;

use crate::connection_pool::ConduitConnectionPool;
use crate::repositories::articles_repository::PostgresArticlesRepository;
use crate::repositories::profiles_repository::PostgresProfilesRepository;
use crate::repositories::tags_repository::PostgresTagsRepository;
use crate::repositories::users_repository::PostgresUsersRepository;
use crate::services::features::articles_service::ConduitArticlesService;
use crate::services::features::profiles_service::ConduitProfilesService;
use crate::services::features::users_service::ConduitUsersService;
use crate::services::utils::argon_security_service::ArgonSecurityService;
use crate::services::utils::jwt_service::JwtService;
use crate::services::utils::postgres_unit_of_work::PostgresUnitOfWork;

#[derive(Clone)]
pub struct ServiceRegister {
    pub users_service: DynUsersService,
    pub token_service: DynTokenService,
    pub profiles_service: DynProfilesService,
    pub articles_service: DynArticlesService,
}

/// A simple service container responsible for managing the various services our API endpoints will pull from through axum extensions.
impl ServiceRegister {
    pub async fn try_new(pool: ConduitConnectionPool, config: Arc<AppConfig>) -> ConduitResult<Self> {
        info!("initializing utility services...");
        let security_service =
            Arc::new(ArgonSecurityService::new(config.clone())) as DynSecurityService;
        let token_service = Arc::new(JwtService::new(config.clone())) as DynTokenService;
        let unit_of_work = Arc::new(PostgresUnitOfWork::try_new(pool.clone()).await?) as DynUnitOfWork;


        info!("utility services initialized, building feature services...");
        let profiles_service = Arc::new(ConduitProfilesService::new(unit_of_work.clone())) as DynProfilesService;
        let articles_service = Arc::new(ConduitArticlesService::new(unit_of_work.clone())) as DynArticlesService;
        let users_service = Arc::new(ConduitUsersService::new(
            unit_of_work.clone(),
            security_service,
            token_service.clone(),
        )) as DynUsersService;

        info!("feature services successfully initialized!");

        Ok(Self {
            users_service,
            token_service,
            profiles_service,
            articles_service,
        })
    }
}
