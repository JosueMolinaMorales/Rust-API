use bson::oid::ObjectId;
use mongodb::Cursor;
use crate::{shared::types::{ApiErrors, User, PasswordRecord, UpdatePasswordRecord, SecretRecord, UpdateSecretRecord}, modules::search_module::SearchParams};

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
    async fn insert_record(&self, record: PasswordRecord) -> Result<ObjectId, ApiErrors>;
    async fn get_record(
        &self,
        record_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<PasswordRecord, ApiErrors>;
    async fn get_all_user_records(
        &self,
        user_id: ObjectId,
    ) -> Result<Cursor<PasswordRecord>, ApiErrors>;
    async fn delete_record(&self, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors>;
    async fn update_record(
        &self,
        updated_record: UpdatePasswordRecord,
        record_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<(), ApiErrors>;

    // Secret Record Methods
    async fn insert_secret(&self, secret_record: SecretRecord) -> Result<ObjectId, ApiErrors>;
    async fn get_secret(
        &self,
        secret_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<SecretRecord, ApiErrors>;
    async fn get_all_secret_records(
        &self,
        user_id: ObjectId,
    ) -> Result<Cursor<SecretRecord>, ApiErrors>;
    async fn delete_secret(&self, secret_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors>;
    async fn update_secret(
        &self,
        updated_secret: UpdateSecretRecord,
        secret_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<(), ApiErrors>;

    // Search
    async fn search_records(
        &self,
        params: SearchParams,
    ) -> Result<Cursor<PasswordRecord>, ApiErrors>;
    async fn search_secrets(&self, params: SearchParams)
        -> Result<Cursor<SecretRecord>, ApiErrors>;
}
