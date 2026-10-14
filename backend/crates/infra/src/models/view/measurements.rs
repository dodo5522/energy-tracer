use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(schema_name = "generation", table_name = "measurements_view")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub system: String,
    pub label: String,
    #[sea_orm(column_type = "Float")]
    pub value: f32,
    pub unit: String,
    pub remark: String,
    pub measured_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
