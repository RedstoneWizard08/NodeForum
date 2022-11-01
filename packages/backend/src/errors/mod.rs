use actix_web::{ResponseError, HttpResponse};
use deadpool_postgres::PoolError;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::Error as PGError;
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum AppError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Self::NotFound => HttpResponse::NotFound().finish(),
            Self::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
