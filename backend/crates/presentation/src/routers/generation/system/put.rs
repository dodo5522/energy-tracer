#[derive(serde::Deserialize, utoipa::IntoParams)]
pub struct UpdateSubSystemQuery {
    pub remark: String,
}
