use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use thiserror::Error;

use crate::util::constants::EnvironmentVariable;

pub mod osu;
pub mod user;

pub type RepoResult<T> = Result<Option<T>, RepoError>;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("{0} is already existed.")]
    Duplicate(String),

    #[error("Error while interacting with the database: {0}")]
    Internal(#[from] mongodb::error::Error),
}

#[derive(Clone)]
pub struct Repo {
    pub user: user::UserRepo,
    pub osu: osu::OsuRepo,
}

impl Repo {
    pub async fn init() -> Self {
        let client = &Client::with_uri_str(EnvironmentVariable::MongoUri.value())
            .await
            .expect("Cannot connect to MongoDB");

        let user = user::UserRepo::init(client).await;
        let osu = osu::OsuRepo::init(client).await;
        Repo { user, osu }
    }
}

pub fn to_object_id(raw_id: &str) -> ObjectId {
    ObjectId::parse_str(raw_id).unwrap_or_default()
}
