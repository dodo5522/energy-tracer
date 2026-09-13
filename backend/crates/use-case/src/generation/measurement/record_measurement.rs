use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, MeasurementRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use layer_domain::entity::MeasurementEntity;
use std::marker::PhantomData;

pub struct RecordMeasurementUseCase<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: MeasurementRepositoryTrait<Tx>,
> {
    repo: R,
    factory: F,
    _marker0: PhantomData<Tx>,
    _marker1: PhantomData<U>,
}

impl<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: MeasurementRepositoryTrait<Tx>,
> ErrorMapperTrait for RecordMeasurementUseCase<Tx, U, F, R>
{
}

impl<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: MeasurementRepositoryTrait<Tx>,
> RecordMeasurementUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn record(self, measurements: Vec<MeasurementEntity>) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        match self.repo.add(uow.ref_tx(), measurements).await {
            Ok(()) => {
                uow.commit().await.map_err(Self::map_db_err)?;
                Ok(())
            }
            Err(e) => {
                uow.rollback().await.map_err(Self::map_db_err)?;
                Err(e)
            }
        }
    }
}
