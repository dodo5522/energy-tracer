use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, UnitOfWorkFactoryTrait, UnitOfWorkTrait, UnitRepositoryTrait,
};
use layer_domain::entity::UnitEntity;
use std::marker::PhantomData;

pub struct FetchUnitsUseCase<
    Tx,
    R: UnitRepositoryTrait<Tx>,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
> {
    repo: R,
    factory: F,
    _marker0: PhantomData<Tx>,
    _marker1: PhantomData<U>,
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: UnitRepositoryTrait<Tx>>
    ErrorMapperTrait for FetchUnitsUseCase<Tx, R, U, F>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: UnitRepositoryTrait<Tx>>
    FetchUnitsUseCase<Tx, R, U, F>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn fetch(self, unit: Option<&String>) -> Result<Vec<UnitEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        Ok(self.repo.find(uow.ref_tx(), unit).await?)
    }
}
