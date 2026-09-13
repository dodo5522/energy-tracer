use super::errors::GenerationError;
use chrono::{DateTime, Utc};
use layer_domain::entity::MeasurementEntity;

/// 発電状況を記録するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait MeasurementRepositoryTrait<Tx> {
    /// 発電状況を記録する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `measurements` - 新規登録する発電状況
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空タプルを返し、失敗時はエラーを返す
    async fn add(
        &self,
        tx: &Tx,
        measurements: Vec<MeasurementEntity>,
    ) -> Result<(), GenerationError>;

    /// 発電状況を取得する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `from` - 取得開始日時
    /// * `to` - 取得開始日時
    /// * `system` - 取得対象サブシステム
    /// * `labels` - 取得対象ラベル（オプション）
    /// # Returns
    /// * `Result<Vec<HistoryEntity>, GenerationRepositoryError>` - 成功時は発電状況のエンティティを返し、失敗時はエラーを返す
    async fn fetch(
        &self,
        tx: &Tx,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        system: String,
        labels: Option<Vec<String>>,
    ) -> Result<Vec<MeasurementEntity>, GenerationError>;

    /// 発電状況を削除する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `id` - 削除する発電状況のID
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    async fn delete(&self, tx: &Tx, id: i64) -> Result<(), GenerationError>;
}
