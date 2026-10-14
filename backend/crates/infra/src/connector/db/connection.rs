use super::DatabasePoolConfig;
use crate::errors::InfraError;
use sea_orm::{Database, DatabaseConnection};

#[derive(Debug, Clone)]
pub struct DatabaseConnector {
    user: String,
    password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
    pool_config: Option<DatabasePoolConfig>,
}

impl DatabaseConnector {
    pub fn new(
        user: String,
        password: String,
        db_host: String,
        db_port: u16,
        db_name: String,
        pool_config: Option<DatabasePoolConfig>,
    ) -> DatabaseConnector {
        Self {
            user,
            password,
            db_host,
            db_port,
            db_name,
            pool_config,
        }
    }

    /// Get the database URL from environment variables.
    ///
    /// # Returns
    /// A `String` representing the database URL in the format:
    /// `postgresql://{DB_USER}:{DB_PASSWORD}@{DB_HOST}:{DB_PORT}/{
    pub fn get_url(&self) -> String {
        let (user, password, db_host, db_port, db_name) = (
            &self.user,
            &self.password,
            &self.db_host,
            &self.db_port,
            &self.db_name,
        );
        format!("postgresql://{user}:{password}@{db_host}:{db_port}/{db_name}")
    }

    /// Get a database connection.
    ///
    /// # Returns
    /// A `DatabaseConnection` instance connected to the database specified in the environment variables.
    pub async fn get_connection(&self) -> Result<DatabaseConnection, InfraError> {
        let url = self.get_url();
        let connection = if let Some(config) = &self.pool_config {
            Database::connect(config.convert_to(url)).await
        } else {
            Database::connect(url).await
        };
        connection.map_err(InfraError::DbFailed)
    }
}
