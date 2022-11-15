use bson::oid::ObjectId;
use mongodb::Cursor;
use crate::{shared::types::{ApiErrors, User, Record, UpdateRecord}, modules::search_module::SearchParams};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TMongoClient: Send + Sync {
    async fn connect(&mut self);
    // Auth Methods
    async fn email_exists(&self, email: &str) -> Result<bool, ApiErrors>;
    async fn username_exists(&self, username: &str) -> Result<bool, ApiErrors>;
    async fn insert_user(&self, user: &User) -> Result<ObjectId, ApiErrors>;
    async fn get_user(&self, username: &str) -> Result<User, ApiErrors>;

    // Password Record Methods
    async fn insert_record(&self, record: Record) -> Result<ObjectId, ApiErrors>;
    async fn get_record(
        &self,
        record_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<Record, ApiErrors>;
    async fn get_all_user_records(
        &self,
        user_id: ObjectId,
    ) -> Result<Cursor<Record>, ApiErrors>;
    async fn delete_record(&self, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors>;
    async fn update_record(
        &self,
        updated_record: UpdateRecord,
        record_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<(), ApiErrors>;

    // Search
    async fn search_records(
        &self,
        params: SearchParams,
    ) -> Result<Cursor<Record>, ApiErrors>;
}
