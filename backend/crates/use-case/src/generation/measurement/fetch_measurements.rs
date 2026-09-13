use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, MeasurementRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use chrono::{DateTime, Utc};
use layer_domain::entity::MeasurementEntity;
use std::marker::PhantomData;

pub struct FetchMeasurementsUseCase<
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
> ErrorMapperTrait for FetchMeasurementsUseCase<Tx, U, F, R>
{
}

impl<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: MeasurementRepositoryTrait<Tx>,
> FetchMeasurementsUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    /// 発電状況記録を取得する
    ///
    /// # Arguments
    /// * `from` - 取得開始日時
    /// * `to` - 取得開始日時
    /// * `system` - 取得対象サブシステム
    /// * `labels` - 取得対象ラベル（オプション）
    /// # Returns
    /// * `Result<Option<MeasurementEntity>, GenerationError>` - 取得した発電状況
    pub async fn fetch(
        self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        system: String,
        labels: Option<Vec<String>>,
    ) -> Result<Vec<MeasurementEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        Ok(self
            .repo
            .fetch(uow.ref_tx(), from, to, system, labels)
            .await?)
    }
}
