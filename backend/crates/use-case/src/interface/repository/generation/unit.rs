use super::errors::GenerationError;
use layer_domain::entity::UnitEntity;

/// 単位を管理するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait UnitRepositoryTrait<Tx> {
    /// 単位を追加する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `unit` - 新規登録する単位
    /// # Returns
    /// * `Result<i64, GenerationRepositoryError>` - 成功時は登録単位のIDを返し、失敗時はエラーを返す
    async fn add(&self, tx: &Tx, unit: UnitEntity) -> Result<i64, GenerationError>;

    /// 登録済みの単位を探す
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `unit` - 情報取得する対象の単位。指定なければ全て取得する。
    /// # Returns
    /// * `Result<UnitEntity, GenerationRepositoryError>` - 成功時は単位のエンティティを返し、失敗時はエラーを返す
    async fn find(
        &self,
        tx: &Tx,
        unit: Option<&String>,
    ) -> Result<Vec<UnitEntity>, GenerationError>;

    /// 単位を更新する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `unit` - 更新する単位のエンティティ
    /// # Returns
    /// * `Result<i64, GenerationRepositoryError>` - 成功時はIDを返し、失敗時はエラーを返す
    async fn update(&self, tx: &Tx, unit: &UnitEntity) -> Result<i64, GenerationError>;

    /// 単位を削除する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `unit` - 削除する単位
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    async fn delete(&self, tx: &Tx, unit: String) -> Result<(), GenerationError>;
}
