use super::errors::GenerationError;
use layer_domain::entity::LabelEntity;

/// ラベル管理リポジトリインターフェース
#[async_trait::async_trait]
pub trait LabelRepositoryTrait<Tx> {
    /// ラベルを追加する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `label` - 新規登録するラベルエンティティ
    /// # Returns
    /// * `Result<i64, GenerationRepositoryError>` - 成功時は登録後のラベルIDを返し、失敗時はエラーを返す
    async fn add(&self, tx: &Tx, label: LabelEntity) -> Result<i64, GenerationError>;

    /// ラベルを取得する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `label` - 取得するラベル。指定なければ全て取得する。
    /// # Returns
    /// * `Result<Vec<LabelRecord>, GenerationRepositoryError>` - 成功時はラベルのエンティティを返し、失敗時はエラーを返す
    async fn find(
        &self,
        tx: &Tx,
        label: Option<&String>,
    ) -> Result<Vec<LabelEntity>, GenerationError>;

    /// ラベルを更新する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `label` - 更新するラベルのエンティティ
    /// # Returns
    /// * `Result<i64, GenerationRepositoryError>` - 成功時はIDを返し、失敗時はエラーを返す
    async fn update(&self, tx: &Tx, label: &LabelEntity) -> Result<i64, GenerationError>;

    /// ラベルを削除する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `label` - 削除するラベル
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    async fn delete(&self, tx: &Tx, label: String) -> Result<(), GenerationError>;
}
