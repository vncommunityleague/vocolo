use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub struct APIResponse<T: Serialize> {
    pub body: Option<T>,
    pub status_code: StatusCode,
}

impl<T> Default for APIResponse<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            body: None,
            status_code: StatusCode::OK,
        }
    }
}

impl<T> APIResponse<T>
where
    T: Serialize,
{
    pub fn body(mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
}

impl<T> IntoResponse for APIResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = match self.body {
            Some(body) => Json(body),
            None => return (self.status_code).into_response(),
        };

        (self.status_code, body).into_response()
    }
}

/// Please use this function to create a response for errors.
/// This will maintain consistency with the error responses across multiple packages.
/// It will create a response with the correct status code and body.
/// The body will be a json object with the following structure:
/// {
///    "error": "error",
///    "error_description": "error_description",
/// }
pub fn into_err_response(status_code: StatusCode, err: &str, err_description: &str) -> Response {
    (
        status_code,
        Json(json!({
            "error": err,
            "error_description": err_description,
        })),
    )
        .into_response()
}

pub async fn handle_404() -> impl IntoResponse {
    into_err_response(
        StatusCode::NOT_FOUND,
        "not_found",
        "The requested resource was not found",
    )
}
