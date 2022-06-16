use std::sync::Arc;

use async_trait::async_trait;

use conduit_core::articles::repository::DynArticlesRepository;
use conduit_core::errors::{ConduitError, ConduitResult};
use conduit_core::profiles::repository::DynProfilesRepository;
use conduit_core::tags::repository::DynTagsRepository;
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::utils::unit_of_work::UnitOfWork;

use crate::connection_pool::{ConduitConnectionPool, ConduitConnectionTransaction};
use crate::repositories::articles_repository::PostgresArticlesRepository;
use crate::repositories::profiles_repository::PostgresProfilesRepository;
use crate::repositories::tags_repository::PostgresTagsRepository;
use crate::repositories::users_repository::PostgresUsersRepository;

pub struct PostgresUnitOfWork {
    transaction: ConduitConnectionTransaction,
    users_repository: DynUsersRepository,
    profiles_repository: DynProfilesRepository,
    articles_repository: DynArticlesRepository,
    tags_repository: DynTagsRepository,
}

impl PostgresUnitOfWork {
    pub async fn try_new(pool: ConduitConnectionPool) -> ConduitResult<Self> {
        let transaction = pool
            .begin()
            .await
            .map_err(|err| ConduitError::InternalServerErrorWithContext(err.to_string()))?;

        let users_repository = Arc::new(PostgresUsersRepository::new(pool.clone())) as DynUsersRepository;
        let profiles_repository = Arc::new(PostgresProfilesRepository::new(pool.clone())) as DynProfilesRepository;
        let articles_repository = Arc::new(PostgresArticlesRepository::new(pool.clone())) as DynArticlesRepository;
        let tags_repository = Arc::new(PostgresTagsRepository::new(pool)) as DynTagsRepository;

        Ok(Self {
            transaction: Arc::new(transaction),
            users_repository,
            profiles_repository,
            articles_repository,
            tags_repository,
        })
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

    async fn commit(mut self) -> ConduitResult<()> {
        let committed = self.transaction
            .clone()
            .commit()
            .await
            .map_err(|err| ConduitError::InternalServerErrorWithContext(err.to_string()));

        // TODO: self is consumed from the `.commit()` above, need some way of rolling back when commit fails
        if committed.is_err() {
            self.transaction
                .rollback()
                .await
                .map_err(|err| ConduitError::InternalServerErrorWithContext(err.to_string()))?;
        }

        Ok(())
    }
}
