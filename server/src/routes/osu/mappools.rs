use actix_web::{delete, error::HttpError, get, HttpResponse, patch, post};

#[get("mappools")]
pub async fn list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("{id}")]
pub async fn get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn delete() -> Result<HttpResponse, HttpError> {
    todo!();
}
