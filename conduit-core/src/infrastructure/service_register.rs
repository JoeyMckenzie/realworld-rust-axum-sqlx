use crate::infrastructure::connection_pool::ConduitConnectionPool;
use crate::users::repository::UsersRepositoryImpl;
use crate::users::service::UsersServiceImpl;
use crate::users::{DynUsersRepository, DynUsersService};
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
