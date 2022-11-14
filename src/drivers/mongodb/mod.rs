pub mod mongo_trait;

use crate::{drivers::mongodb::mongo_trait::TMongoClient, shared::types::{Record, UpdateRecord}};
use bson::{doc, oid::ObjectId, Document};
use mongodb::{
    options::{ClientOptions, FindOptions},
    Client, Cursor,
};

use crate::{
    modules::search_module::SearchParams,
    shared::{
        env_config::{get_db_name, get_db_uri},
        types::{
            ApiErrors, PasswordRecord, SecretRecord, UpdateSecretRecord, User,
        },
    },
};

pub struct MongoClient {
    client: Option<mongodb::Client>,
}

impl Default for MongoClient {
    fn default() -> Self {
        Self::new()
    }
}
impl MongoClient {
    pub fn new() -> MongoClient {
        MongoClient { client: None }
    }
    /**
     * Get the client
     */
    pub fn get_client(&self) -> &mongodb::Client {
        match &self.client {
            Some(val) => val,
            None => panic!("Connect to client not established!"),
        }
    }
}

#[async_trait]
impl TMongoClient for MongoClient {
    /**
     * Connect to the Database
     */
    async fn connect(&mut self) {
        let db_uri = get_db_uri();
        let client_options = ClientOptions::parse(db_uri)
            .await
            .expect("There was an error parsing the DB_URI");

        let client = Client::with_options(client_options)
            .expect("There was an error connecting to the database");

        println!("Connection to mongodb established!");

        self.client = Some(client);
    }

    async fn search_records(
        &self,
        params: SearchParams,
    ) -> Result<Cursor<PasswordRecord>, ApiErrors> {
        let mut filter = doc! {
            "user_id": params.user_id
        };

        if let Some(service) = params.service {
            filter.insert("service", service);
        };

        let find_options = FindOptions::builder()
            .limit(if params.limit.is_some() {
                params.limit
            } else {
                Some(10)
            })
            .skip(params.page)
            .build();

        let res = self
            .get_client()
            .database(&get_db_name())
            .collection::<PasswordRecord>("records")
            .find(filter, find_options)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?;

        Ok(res)
    }

    async fn search_secrets(
        &self,
        params: SearchParams,
    ) -> Result<Cursor<SecretRecord>, ApiErrors> {
        let mut filter = doc! {
            "user_id": params.user_id
        };

        if let Some(key) = params.key {
            filter.insert("key", key);
        };

        let find_options = FindOptions::builder()
            .limit(if params.limit.is_some() {
                params.limit
            } else {
                Some(10)
            })
            .skip(params.page)
            .build();

        let res = self
            .get_client()
            .database(&get_db_name())
            .collection::<SecretRecord>("secrets")
            .find(filter, find_options)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?;
        Ok(res)
    }

    async fn email_exists(&self, email: &str) -> Result<bool, ApiErrors> {
        let count = self
            .get_client()
            .database(&get_db_name())
            .collection::<User>("users")
            .count_documents(doc! { "email": email }, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?;

        Ok(count != 0)
    }

    async fn username_exists(&self, username: &str) -> Result<bool, ApiErrors> {
        let count = self
            .get_client()
            .database(&get_db_name())
            .collection::<User>("users")
            .count_documents(doc! { "username": username }, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?;

        Ok(count != 0)
    }

    async fn insert_user(&self, user: &User) -> Result<ObjectId, ApiErrors> {
        let res = self
            .get_client()
            .database(&get_db_name())
            .collection::<User>("users")
            .insert_one(user, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiErrors::ServerError("Error".to_string()))?;
        Ok(res)
    }

    async fn get_user(&self, username: &str) -> Result<User, ApiErrors> {
        let user = self
            .get_client()
            .database(&get_db_name())
            .collection::<User>("users")
            .find_one(doc! { "username": username }, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?
            .ok_or_else(|| ApiErrors::BadRequest("User not found".to_string()))?;
        Ok(user)
    }

    async fn get_all_user_records(
        &self,
        user_id: ObjectId,
    ) -> Result<Cursor<Record>, ApiErrors> {
        let res = self
            .get_client()
            .database(&get_db_name())
            .collection::<Record>("records")
            .find(doc! { "user_id": user_id }, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?;
        Ok(res)
    }

    async fn insert_record(&self, record: Record) -> Result<ObjectId, ApiErrors> {
        let obj_id = self
            .get_client()
            .database(&get_db_name())
            .collection::<Record>("records")
            .insert_one(record, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiErrors::ServerError("Error converting object id".to_string()))?;

        Ok(obj_id)
    }

    async fn get_record(
        &self,
        record_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<Record, ApiErrors> {
        let record = self
            .get_client()
            .database(&get_db_name())
            .collection::<Record>("records")
            .find_one(doc! { "_id": record_id, "user_id": user_id }, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?
            .ok_or_else(|| ApiErrors::NotFound("Record not found".to_string()))?;

        Ok(record)
    }

    async fn delete_record(&self, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
        self.get_client()
            .database(&get_db_name())
            .collection::<PasswordRecord>("records")
            .find_one_and_delete(doc! {"_id": record_id, "user_id": user_id}, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?
            .ok_or_else(|| ApiErrors::NotFound("Record not found".to_string()))?;
        Ok(())
    }

    async fn update_record(
        &self,
        updated_record: UpdateRecord,
        record_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<(), ApiErrors> {
        let mut update = Document::new();
        if let Some(email) = updated_record.email {
            update.insert("email", email);
        }
        if let Some(password) = updated_record.password {
            update.insert("password", password);
        }
        if let Some(username) = updated_record.username {
            update.insert("username", username);
        }
        if let Some(service) = updated_record.service {
            update.insert("service", service);
        }
        if let Some(key) = updated_record.key {
            update.insert("key", key);
        }
        if let Some(secret) = updated_record.secret {
            update.insert("secret", secret);
        }

        self.get_client()
            .database(&get_db_name())
            .collection::<PasswordRecord>("records")
            .find_one_and_update(
                doc! { "_id": record_id, "user_id": user_id },
                doc! { "$set": update },
                None,
            )
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?;

        Ok(())
    }

    async fn insert_secret(&self, secret_record: SecretRecord) -> Result<ObjectId, ApiErrors> {
        let object_id = self
            .get_client()
            .database(&get_db_name())
            .collection("secrets")
            .insert_one(secret_record, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiErrors::ServerError("Error converting object id".to_string()))?;

        Ok(object_id)
    }

    async fn get_all_secret_records(
        &self,
        user_id: ObjectId,
    ) -> Result<Cursor<SecretRecord>, ApiErrors> {
        let cursor = self
            .get_client()
            .database(&get_db_name())
            .collection::<SecretRecord>("secrets")
            .find(doc! { "user_id": user_id }, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?;

        Ok(cursor)
    }

    async fn get_secret(
        &self,
        secret_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<SecretRecord, ApiErrors> {
        let secret = self
            .get_client()
            .database(&get_db_name())
            .collection::<SecretRecord>("secrets")
            .find_one(doc! { "_id": secret_id, "user_id": user_id }, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?
            .ok_or_else(|| ApiErrors::NotFound("Secret Record Not Found".to_string()))?;
        Ok(secret)
    }

    async fn delete_secret(&self, secret_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
        self.get_client()
            .database(&get_db_name())
            .collection::<SecretRecord>("secrets")
            .find_one_and_delete(doc! { "_id": secret_id, "user_id": user_id }, None)
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?
            .ok_or_else(|| ApiErrors::NotFound("Secret Record Not Found".to_string()))?;
        Ok(())
    }

    async fn update_secret(
        &self,
        updated_secret: UpdateSecretRecord,
        secret_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<(), ApiErrors> {
        let mut updated_document = Document::new();

        if let Some(key) = updated_secret.key {
            updated_document.insert("key", key);
        }
        if let Some(secret) = updated_secret.secret {
            updated_document.insert("secret", secret);
        }

        self.get_client()
            .database(&get_db_name())
            .collection::<SecretRecord>("secrets")
            .find_one_and_update(
                doc! { "_id": secret_id, "user_id": user_id },
                updated_document,
                None,
            )
            .await
            .map_err(|err| ApiErrors::ServerError(err.to_string()))?;

        Ok(())
    }
}
