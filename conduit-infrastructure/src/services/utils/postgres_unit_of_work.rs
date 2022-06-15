use std::sync::Arc;

use async_trait::async_trait;

use conduit_core::articles::repository::DynArticlesRepository;
use conduit_core::errors::{ConduitError, ConduitResult};
use conduit_core::profiles::repository::DynProfilesRepository;
use conduit_core::tags::repository::DynTagsRepository;
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::utils::unit_of_work::UnitOfWork;

use crate::connection_pool::ConduitConnectionPool;
use crate::repositories::articles_repository::PostgresArticlesRepository;
use crate::repositories::profiles_repository::PostgresProfilesRepository;
use crate::repositories::tags_repository::PostgresTagsRepository;
use crate::repositories::users_repository::PostgresUsersRepository;

pub struct PostgresUnitOfWork {
    pool: ConduitConnectionPool,
    pub users_repository: DynUsersRepository,
    pub profiles_repository: DynProfilesRepository,
    pub articles_repository: DynArticlesRepository,
    pub tags_repository: DynTagsRepository,
}

impl PostgresUnitOfWork {
    pub fn new(pool: ConduitConnectionPool) -> Self {
        Self {
            pool: pool.clone(),
            users_repository: Arc::new(PostgresUsersRepository::new(pool.clone()))
                as DynUsersRepository,
            profiles_repository: Arc::new(PostgresProfilesRepository::new(pool.clone()))
                as DynProfilesRepository,
            articles_repository: Arc::new(PostgresArticlesRepository::new(pool.clone()))
                as DynArticlesRepository,
            tags_repository: Arc::new(PostgresTagsRepository::new(pool)) as DynTagsRepository,
        }
    }
}

#[async_trait]
impl UnitOfWork for PostgresUnitOfWork {
    fn users_repository(&self) -> DynUsersRepository {
        self.users_repository.clone()
    }

    fn profiles_repository(&self) -> DynProfilesRepository {
        self.profiles_repository.clone()
    }

    fn articles_repository(&self) -> DynArticlesRepository {
        self.articles_repository.clone()
    }

    fn tags_repository(&self) -> DynTagsRepository {
        self.tags_repository.clone()
    }

    async fn begin(&self) -> ConduitResult<()> {
        let test = self
            .pool
            .begin()
            .await
            .map_err(|err| ConduitError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(())
    }

    async fn commit(&self) -> ConduitResult<()> {
        Ok(())
    }

    async fn rollback(&self) -> ConduitResult<()> {
        todo!()
    }
}
