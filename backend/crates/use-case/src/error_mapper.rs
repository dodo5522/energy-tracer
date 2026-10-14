use crate::interface::GenerationError;

pub trait ErrorMapperTrait {
    fn map_db_err(e: std::io::Error) -> GenerationError {
        GenerationError::DbError(format!("{e}"))
    }
}
