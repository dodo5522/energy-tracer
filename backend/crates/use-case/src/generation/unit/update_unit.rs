use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, UnitOfWorkFactoryTrait, UnitOfWorkTrait, UnitRepositoryTrait,
};
use layer_domain::entity::UnitEntity;
use std::marker::PhantomData;

pub struct UpdateUnitUseCase<
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
    ErrorMapperTrait for UpdateUnitUseCase<Tx, R, U, F>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: UnitRepositoryTrait<Tx>>
    UpdateUnitUseCase<Tx, R, U, F>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn update(self, input: UnitEntity) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;

        if let Err(e) = self.repo.update(uow.ref_tx(), &input).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }
}
