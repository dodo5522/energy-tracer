use crate::schemas::GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Unit {
    Schema,
    Table,
    Id,
    Unit,
    Remark,
    CreatedAt,
}

impl Iden for Unit {
    fn unquoted(&self) -> &str {
        match self {
            Self::Schema => GENERATION,
            Self::Table => "units",
            Self::Id => "id",
            Self::Unit => "unit",
            Self::Remark => "remark",
            Self::CreatedAt => "created_at",
        }
    }
}
