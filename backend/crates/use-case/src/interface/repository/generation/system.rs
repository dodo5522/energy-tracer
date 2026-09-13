use super::errors::GenerationError;
use layer_domain::entity::SystemEntity;

/// システムを記録するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait SubSystemRepositoryTrait<Tx> {
    /// システムを追加する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `system` - 新規登録するシステム
    /// # Returns
    /// * `Result<i64, GenerationRepositoryError>` - 成功時はIDを返し、失敗時はエラーを返す
    async fn add(&self, tx: &Tx, system: SystemEntity) -> Result<i64, GenerationError>;

    /// システムを取得する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `system` - 情報取得する対象のシステム。指定なければ全て取得する。
    /// # Returns
    /// * `Result<Vec<GroupRecord>, GenerationRepositoryError>` - 成功時はシステムのエンティティを返し、失敗時はエラーを返す
    async fn find(
        &self,
        tx: &Tx,
        system: Option<&String>,
    ) -> Result<Vec<SystemEntity>, GenerationError>;

    /// システムを更新する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `system` - 更新する対象のシステム
    /// # Returns
    /// * `Result<i64, GenerationRepositoryError>` - 成功時はIDを返し、失敗時はエラーを返す
    async fn update(&self, tx: &Tx, system: &SystemEntity) -> Result<i64, GenerationError>;

    /// システムを削除する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `system` - 削除するシステム
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    async fn delete(&self, tx: &Tx, system: String) -> Result<(), GenerationError>;
}
