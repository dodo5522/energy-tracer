use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, UnitOfWorkFactoryTrait, UnitOfWorkTrait, UnitRepositoryTrait,
};
use std::marker::PhantomData;

pub struct DeleteUnitUseCase<
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
    ErrorMapperTrait for DeleteUnitUseCase<Tx, R, U, F>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: UnitRepositoryTrait<Tx>>
    DeleteUnitUseCase<Tx, R, U, F>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn delete(self, unit: impl AsRef<str>) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        if let Err(e) = self.repo.delete(uow.ref_tx(), unit.as_ref().into()).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }
}
