use crate::infrastructure::connection_pool::ConduitConnectionPool;
use crate::repositories::ConduitConnectionPool;
use crate::services::users_service::UsersServiceImpl;
use crate::users::repository::{DynUsersRepository, UsersRepositoryImpl};
use crate::users::service::DynUsersService;
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::users::service::DynUsersService;
use std::sync::Arc;

pub struct ServiceRegister {
    pub users_service: DynUsersService,
}

impl ServiceRegister {
    pub fn new(pool: ConduitConnectionPool) -> Self {
        let arc_pool = Arc::new(pool);

        let users_repository = Arc::new(UsersRepositoryImpl::new(arc_pool)) as DynUsersRepository;
        let users_service = Arc::new(UsersServiceImpl::new(users_repository)) as DynUsersService;

        ServiceRegister { users_service }
    }
}
