use layer_domain::value_object::UnitError;
use layer_use_case::interface::GenerationError;
use sea_orm::DbErr;

pub trait ErrorMapperTrait {
    fn map_unknown_err<E: std::fmt::Display>(e: E) -> GenerationError {
        GenerationError::Unknown(format!("{e}"))
    }

    fn map_db_to_generation_error(e: DbErr) -> GenerationError {
        match e {
            DbErr::RecordNotUpdated => GenerationError::NotFound(format!("{e}")),
            DbErr::RecordNotFound(msg) => GenerationError::NotFound(msg),
            _ => GenerationError::DbError(format!("{e}")),
        }
    }

    fn map_invalid_unit(e: UnitError) -> GenerationError {
        match e {
            UnitError::Invalid(unit) => GenerationError::InvalidUnit(unit),
            _ => GenerationError::InvalidUnit(String::new()),
        }
    }
}
