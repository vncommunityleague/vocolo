use axum::http::StatusCode;
use axum::Json;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Custom(String),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let description = self.to_string();
        let (status_code, error_message) = match self {
            Error::Database(..) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            Error::NotFound(..) => (StatusCode::NOT_FOUND, "not_found"),
            Error::Custom(_) => (StatusCode::INTERNAL_SERVER_ERROR, "custom_error"),
        };

        (
            status_code,
            Json(serde_json::json!({
                "error": &error_message,
                "error_description": &description,
            })),
        )
            .into_response()
    }
}
