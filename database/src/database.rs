use bson::{doc, oid::ObjectId, Document};
use futures::StreamExt;
use mongodb::{
    options::{CountOptions, FindOneOptions, FindOptions},
    results::{DeleteResult, InsertOneResult},
    Client, Collection,
};
use serde::{de::DeserializeOwned, Serialize};
use tracing::info;

use vocolo_internal::Result;

use crate::to_vocolo_error;

pub async fn connect_to_db() -> Result<Database> {
    info!("Connecting to MongoDB");

    let uri = std::env::var("MONGO_URL").expect("`MONGO_URL` must be set");
    let client = Client::with_uri_str(uri).await.map_err(to_vocolo_error)?;

    let db_name = std::env::var("MONGO_DATABASE_NAME").expect("`MONGO_DATABASE_NAME` must be set");
    let mongodb = Database(client, db_name);

    Ok(mongodb)
}

#[derive(Clone)]
pub struct Database(pub Client, pub String);

impl std::ops::Deref for Database {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Database {
    fn db(&self) -> mongodb::Database {
        self.database(&self.1)
    }

    pub fn col<T>(&self, collection: &str) -> Collection<T> {
        self.db().collection(collection)
    }

    pub async fn insert_one<T: Serialize>(
        &self,
        collection: &str,
        document: T,
    ) -> Result<InsertOneResult> {
        self.col(collection)
            .insert_one(document, None)
            .await
            .map_err(to_vocolo_error)
    }

    pub async fn count(&self, collection: &'static str, projection: Document) -> Result<u64> {
        self.count_with_options(collection, projection, None).await
    }

    pub async fn count_with_options(
        &self,
        collection: &'static str,
        filter: Document,
        options: Option<CountOptions>,
    ) -> Result<u64> {
        self.col::<Document>(collection)
            .count_documents(filter, options)
            .await
            .map_err(to_vocolo_error)
    }

    pub async fn exists(&self, collection: &'static str, filter: Document) -> Result<bool> {
        Ok(self.count(collection, filter).await? > 0)
    }

    /// Find multiple documents in a collection with options
    pub async fn find_with_options<O, T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        collection: &'static str,
        projection: Document,
        options: O,
    ) -> Result<Vec<T>>
    where
        O: Into<Option<FindOptions>>,
    {
        Ok(self
            .col::<T>(collection)
            .find(projection, options)
            .await
            .map_err(to_vocolo_error)?
            .filter_map(|s| async {
                if cfg!(debug_assertions) {
                    // Hard fail on invalid documents
                    Some(s.unwrap())
                } else {
                    s.ok()
                }
            })
            .collect::<Vec<T>>()
            .await)
    }

    /// Find multiple documents in a collection
    pub async fn find<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        collection: &'static str,
        projection: Document,
    ) -> Result<Vec<T>> {
        self.find_with_options(collection, projection, None).await
    }

    /// Find one document with options
    pub async fn find_one_with_options<O, T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        collection: &'static str,
        projection: Document,
        options: O,
    ) -> Result<Option<T>>
    where
        O: Into<Option<FindOneOptions>>,
    {
        self.col::<T>(collection)
            .find_one(projection, options)
            .await
            .map_err(to_vocolo_error)
    }

    /// Find one document
    pub async fn find_one<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        collection: &'static str,
        projection: Document,
    ) -> Result<Option<T>> {
        self.find_one_with_options(collection, projection, None)
            .await
    }

    /// Delete one document by the given projection
    pub async fn delete_one(
        &self,
        collection: &'static str,
        projection: Document,
    ) -> Result<DeleteResult> {
        self.col::<Document>(collection)
            .delete_one(projection, None)
            .await
            .map_err(to_vocolo_error)
    }

    /// Delete one document by the given ID
    pub async fn delete_one_by_id(
        &self,
        collection: &'static str,
        id: &ObjectId,
    ) -> Result<DeleteResult> {
        self.delete_one(
            collection,
            doc! {
                "_id": id
            },
        )
        .await
    }
}
