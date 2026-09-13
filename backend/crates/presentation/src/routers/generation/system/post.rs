#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct SubSystemPostRequest {
    /// 発電サブシステムの種類
    pub system: String,
    /// 備考
    pub remark: String,
}
