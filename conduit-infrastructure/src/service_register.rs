use std::sync::Arc;

use tracing::info;

use conduit_core::config::AppConfig;
use conduit_core::profiles::repository::DynProfilesRepository;
use conduit_core::profiles::service::DynProfilesService;
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::users::service::DynUsersService;
use conduit_core::utils::security_service::DynSecurityService;
use conduit_core::utils::token_service::DynTokenService;

use crate::connection_pool::ConduitConnectionPool;
use crate::repositories::profiles_repository::PostgresProfilesRepository;
use crate::repositories::users_repository::PostgresUsersRepository;
use crate::services::features::profiles_service::ConduitProfilesService;
use crate::services::features::users_service::ConduitUsersService;
use crate::services::utils::argon_security_service::ArgonSecurityService;
use crate::services::utils::jwt_service::JwtService;

pub struct ServiceRegister {
    pub users_service: DynUsersService,
    pub token_service: DynTokenService,
    pub profiles_service: DynProfilesService,
}

impl ServiceRegister {
    pub fn new(pool: ConduitConnectionPool, config: AppConfig) -> Self {
        let arc_pool = Arc::new(pool);
        let arc_config = Arc::new(config);

        info!("initializing utility services...");
        let security_service =
            Arc::new(ArgonSecurityService::new(arc_config.clone())) as DynSecurityService;
        let token_service = Arc::new(JwtService::new(arc_config)) as DynTokenService;

        info!("utility services initialized, building feature services...");
        let users_repository =
            Arc::new(PostgresUsersRepository::new(arc_pool.clone())) as DynUsersRepository;
        let users_service = Arc::new(ConduitUsersService::new(
            users_repository.clone(),
            security_service,
            token_service.clone(),
        )) as DynUsersService;

        let profiles_repository =
            Arc::new(PostgresProfilesRepository::new(arc_pool)) as DynProfilesRepository;
        let profiles_service = Arc::new(ConduitProfilesService::new(
            users_repository,
            profiles_repository,
        )) as DynProfilesService;

        ServiceRegister {
            users_service,
            token_service,
            profiles_service,
        }
    }
}
