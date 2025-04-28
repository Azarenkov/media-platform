use sqlx::PgPool;

use crate::domain::{
    entities::user::User,
    repositories::{errors::UserRepositoryError, user_repository_abstract::UserRepositoryAbstract},
};

// impl From<sqlx::Error> for UserRepositoryError {
//     fn from(value: sqlx::Error) -> Self {
//         match value {
//             sqlx::Error::Configuration(error) => todo!(),
//             sqlx::Error::InvalidArgument(_) => todo!(),
//             sqlx::Error::Database(database_error) => todo!(),
//             sqlx::Error::Io(error) => todo!(),
//             sqlx::Error::Tls(error) => todo!(),
//             sqlx::Error::Protocol(_) => todo!(),
//             sqlx::Error::RowNotFound => todo!(),
//             sqlx::Error::TypeNotFound { type_name } => todo!(),
//             sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
//             sqlx::Error::ColumnNotFound(_) => todo!(),
//             sqlx::Error::ColumnDecode { index, source } => todo!(),
//             sqlx::Error::Encode(error) => todo!(),
//             sqlx::Error::Decode(error) => todo!(),
//             sqlx::Error::AnyDriverError(error) => todo!(),
//             sqlx::Error::PoolTimedOut => todo!(),
//             sqlx::Error::PoolClosed => todo!(),
//             sqlx::Error::WorkerCrashed => todo!(),
//             sqlx::Error::Migrate(migrate_error) => todo!(),
//             sqlx::Error::InvalidSavePointStatement => todo!(),
//             sqlx::Error::BeginFailed => todo!(),
//             _ => todo!(),
//         }
//     }
// }

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepositoryAbstract for UserRepository {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
        let query = "INSERT INTO users (email, username, password) VALUES ($1, $2, $3)";

        sqlx::query(query)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password)
            .execute(&self.pool)
            .await
            .map_err(|e| UserRepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
