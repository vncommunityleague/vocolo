#![allow(async_fn_in_trait)]
use bson::oid::ObjectId;

mod bridge;
pub use bridge::*;
mod database;
pub use database::*;
mod models;
pub use models::*;

pub(crate) fn to_vocolo_error(source: mongodb::error::Error) -> vocolo_internal::Error {
    vocolo_internal::Error::Database(source.to_string())
}

pub const FAKE_OID: ObjectId = ObjectId::from_bytes([0; 12]);

pub fn str_to_oid(id: &str) -> ObjectId {
    ObjectId::parse_str(id).unwrap_or(FAKE_OID)
}
