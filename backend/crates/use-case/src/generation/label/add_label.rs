use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, LabelRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use layer_domain::entity::LabelEntity;
use std::marker::PhantomData;

pub struct AddLabelUseCase<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: LabelRepositoryTrait<Tx>,
> {
    repo: R,
    factory: F,
    _marker0: PhantomData<Tx>,
    _marker1: PhantomData<U>,
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: LabelRepositoryTrait<Tx>>
    ErrorMapperTrait for AddLabelUseCase<Tx, U, F, R>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: LabelRepositoryTrait<Tx>>
    AddLabelUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn add(self, input: LabelEntity) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        if let Err(e) = self.repo.add(uow.ref_tx(), input).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }
}
