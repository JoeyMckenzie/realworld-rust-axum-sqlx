use std::sync::Arc;

use tracing::info;

use conduit_core::config::AppConfig;
use conduit_core::services::security_service::DynSecurityService;
use conduit_core::services::token_service::DynTokenService;
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::users::service::DynUsersService;

use crate::connection_pool::ConduitConnectionPool;
use crate::repositories::users_repository::PostgresUsersRepository;
use crate::services::features::users_service::ConduitUsersService;
use crate::services::utils::argon_security_service::ArgonSecurityService;
use crate::services::utils::jwt_service::JwtService;

pub struct ServiceRegister {
    pub users_service: DynUsersService,
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
            Arc::new(PostgresUsersRepository::new(arc_pool)) as DynUsersRepository;
        let users_service = Arc::new(ConduitUsersService::new(
            users_repository,
            security_service,
            token_service,
        )) as DynUsersService;

        ServiceRegister { users_service }
    }
}
