use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, SubSystemRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use layer_domain::entity::SystemEntity;
use std::marker::PhantomData;

pub struct FetchSystemUseCase<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: SubSystemRepositoryTrait<Tx>,
> {
    repo: R,
    factory: F,
    _marker0: PhantomData<Tx>,
    _marker1: PhantomData<U>,
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: SubSystemRepositoryTrait<Tx>>
    ErrorMapperTrait for FetchSystemUseCase<Tx, U, F, R>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: SubSystemRepositoryTrait<Tx>>
    FetchSystemUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn fetch(
        self,
        system: Option<&String>,
    ) -> Result<Vec<SystemEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        Ok(self.repo.find(uow.ref_tx(), system).await?)
    }
}
