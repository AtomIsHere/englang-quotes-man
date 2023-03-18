use axum::{response::IntoResponse, http::StatusCode, Json};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db
    }
}
