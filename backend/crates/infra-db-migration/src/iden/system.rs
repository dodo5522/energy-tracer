use crate::schemas::GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum System {
    Schema,
    Table,
    Id,
    System,
    Remark,
    CreatedAt,
}

impl Iden for System {
    fn unquoted(&self) -> &str {
        match self {
            Self::Schema => GENERATION,
            Self::Table => "systems",
            Self::Id => "id",
            Self::System => "system",
            Self::Remark => "remark",
            Self::CreatedAt => "created_at",
        }
    }
}
