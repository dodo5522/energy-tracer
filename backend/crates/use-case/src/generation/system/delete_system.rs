use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, SubSystemRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use std::marker::PhantomData;

pub struct DeleteSystemUseCase<
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
    ErrorMapperTrait for DeleteSystemUseCase<Tx, U, F, R>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: SubSystemRepositoryTrait<Tx>>
    DeleteSystemUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn delete(self, system: impl AsRef<str> + Send) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        if let Err(e) = self.repo.delete(uow.ref_tx(), system.as_ref().into()).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }
}
