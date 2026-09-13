use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, LabelRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use layer_domain::entity::LabelEntity;
use std::marker::PhantomData;

pub struct FetchLabelsUseCase<
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
    ErrorMapperTrait for FetchLabelsUseCase<Tx, U, F, R>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: LabelRepositoryTrait<Tx>>
    FetchLabelsUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn fetch(self, label: Option<&String>) -> Result<Vec<LabelEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        Ok(self.repo.find(uow.ref_tx(), label).await?)
    }
}
