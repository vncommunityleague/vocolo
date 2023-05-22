use crate::repository::{RepoError, RepoResult};
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::{doc, from_bson, Bson, Document};
use futures::{StreamExt, TryStreamExt};
use mongodb::options::{
    FindOneAndUpdateOptions, FindOneOptions, FindOptions, ReturnDocument, UpdateOptions,
};
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{Collection, Cursor};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
pub trait ModelExt {
    type T: DeserializeOwned + Unpin + Send + Sync;

    async fn create(col: Collection<Self::T>, mut model: Self::T) -> RepoResult<Self::T> {
        // model.save(connection, None).await.map_err(Error::Wither)?;

        // TODO: Impl this

        Ok(model)
    }

    async fn find_by_id(
        col: Collection<Self::T>,
        id: &ObjectId,
    ) -> RepoResult<Option<Self::T>> {
        Self::find_one(col, doc! { "_id": id }, None).await
    }

    async fn find_one<O>(
        col: Collection<Self::T>,
        filter: Document,
        options: O,
    ) -> RepoResult<Option<Self::T>>
    where
        O: Into<Option<FindOneOptions>> + Send,
    {
        col.find_one(filter, options)
            .await
            .map_err(RepoError::Internal)
    }

    async fn find<O>(
        col: Collection<Self::T>,
        filter: Document,
        options: O,
    ) -> RepoResult<Vec<Self::T>>
    where
        O: Into<Option<FindOptions>> + Send,
    {
        col.find(filter, options)
            .await
            .map_err(RepoError::Internal)?
            .try_collect::<Vec<Self::T>>()
            .await
            .map_err(RepoError::Internal)
    }

    async fn find_and_count<O>(
        &self,
        col: Collection<Self::T>,
        filter: Document,
        options: O,
    ) -> RepoResult<(Vec<Self::T>, u64)>
    where
        O: Into<Option<FindOptions>> + Send,
    {
        let count = col
            .count_documents(filter.clone(), None)
            .await
            .map_err(RepoError::Internal)?;

        let items = col
            .find(filter, options)
            .await
            .map_err(RepoError::Internal)?
            .try_collect::<Vec<Self::T>>()
            .await
            .map_err(RepoError::Internal)?;

        Ok((items, count))
    }

    async fn cursor<O>(
        col: Collection<Self::T>,
        filter: Document,
        options: O,
    ) -> RepoResult<Cursor<Self::T>>
    where
        O: Into<Option<FindOptions>> + Send,
    {
        col.find(filter, options).await.map_err(RepoError::Internal)
    }

    async fn find_one_and_update(
        col: Collection<Self::T>,
        filter: Document,
        update: Document,
    ) -> RepoResult<Option<Self::T>> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        col.find_one_and_update(filter, update, options)
            .await
            .map_err(RepoError::Internal)
    }

    async fn update_one<O>(
        col: Collection<Self::T>,
        filter: Document,
        update: Document,
        options: O,
    ) -> RepoResult<UpdateResult>
    where
        O: Into<Option<UpdateOptions>> + Send,
    {
        col.update_one(filter, update, options)
            .await
            .map_err(RepoError::Internal)
    }

    async fn update_many<O>(
        col: Collection<Self::T>,
        filter: Document,
        update: Document,
        options: O,
    ) -> RepoResult<UpdateResult>
    where
        O: Into<Option<UpdateOptions>> + Send,
    {
        col.update_many(filter, update, options)
            .await
            .map_err(RepoError::Internal)
    }

    async fn find_one_and_delete(
        col: Collection<Self::T>,
        filter: Document,
    ) -> RepoResult<Option<Self::T>> {
        col.find_one_and_delete(filter, None)
            .await
            .map_err(RepoError::Internal)
    }

    async fn delete_one(col: Collection<Self::T>, filter: Document) -> RepoResult<DeleteResult> {
        col.delete_one(filter, None)
            .await
            .map_err(RepoError::Internal)
    }

    async fn delete_many(col: Collection<Self::T>, filter: Document) -> RepoResult<DeleteResult> {
        col.delete_many(filter, None)
            .await
            .map_err(RepoError::Internal)
    }

    async fn count(col: Collection<Self::T>, filter: Document) -> RepoResult<u64> {
        col.count_documents(filter, None)
            .await
            .map_err(RepoError::Internal)
    }

    async fn exists(&mut self, col: Collection<Self::T>, filter: Document) -> RepoResult<bool> {
        col.count_documents(filter, None)
            .await
            .map_err(RepoError::Internal)
            .map(|count| count > 0)
    }

    async fn aggregate<A>(col: Collection<Self::T>, pipeline: Vec<Document>) -> RepoResult<Vec<A>>
    where
        A: Serialize + DeserializeOwned,
    {
        let documents = col
            .aggregate(pipeline, None)
            .await
            .map_err(RepoError::Internal)?
            .try_collect::<Vec<Document>>()
            .await
            .map_err(RepoError::Internal)?;

        let documents = documents
            .into_iter()
            .map(|document| from_bson::<A>(Bson::Document(document)))
            .collect::<Result<Vec<A>, bson::de::Error>>()
            .map_err(RepoError::SerializeResponse)?;

        Ok(documents)
    }
    //
    // async fn sync_indexes() -> Result<(), Error> {
    //     let connection = CONNECTION.get().await;
    //     Self::T::sync(connection).await.map_err(Error::Wither)
    // }
}
