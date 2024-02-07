use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),

    #[error("{0}")]
    Model(#[from] vocolo_models::error::Error),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Error: {0}")]
    Custom(String),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let description = self.to_string();
        let (status_code, error_message) = match self {
            Error::Database(..) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            Error::Model(..) => (StatusCode::INTERNAL_SERVER_ERROR, "model_error"),
            Error::NotFound(..) => (StatusCode::NOT_FOUND, "not_found"),
            Error::Custom(_) => (StatusCode::INTERNAL_SERVER_ERROR, "custom_error"),
        };

        vocolo_core::into_err_response(status_code, error_message, &description)
    }
}
