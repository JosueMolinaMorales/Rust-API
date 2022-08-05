use bson::{oid::ObjectId, Document, doc};
use mongodb::{ Client, options::ClientOptions, Cursor };

use crate::shared::{types::{ApiErrors, User, PasswordRecord, UpdatePasswordRecord, SecretRecord, UpdateSecretRecord}, env_config::{get_db_uri, get_db_name}};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TMongoClient: Send + Sync {
    async fn connect(&mut self);
    // Auth Methods
    async fn email_exists(&self, email: &String) -> Result<bool, ApiErrors>;
    async fn username_exists(&self, username: &String) -> Result<bool, ApiErrors>;
    async fn insert_user(&self, user: &User) -> Result<ObjectId, ApiErrors>;
    async fn get_user(&self, username: &String) -> Result<User, ApiErrors>;

    // Password Record Methods
    async fn insert_record(&self, record: PasswordRecord) -> Result<ObjectId, ApiErrors>;
    async fn get_record(&self, record_id: ObjectId, user_id: ObjectId) -> Result<PasswordRecord, ApiErrors>;
    async fn get_all_user_records(&self, user_id: ObjectId) -> Result<Cursor<PasswordRecord>, ApiErrors>;
    async fn delete_record(&self, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors>;
    async fn update_record(
        &self,  
        updated_record: UpdatePasswordRecord, 
        record_id: ObjectId, 
        user_id: ObjectId
    ) -> Result<(), ApiErrors>;

    // Secret Record Methods
    async fn insert_secret(&self, secret_record: SecretRecord) -> Result<ObjectId, ApiErrors>;
    async fn get_secret(&self, secret_id: ObjectId, user_id: ObjectId) -> Result<SecretRecord, ApiErrors>;
    async fn get_all_secret_records(&self, user_id: ObjectId) -> Result<Cursor<SecretRecord>, ApiErrors>;
    async fn delete_secret(&self, secret_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors>;
    async fn update_secret(&self, updated_secret: UpdateSecretRecord, secret_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors>;
    
}

pub struct MongoClient {
    client: Option<mongodb::Client>
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
            Some(val) => return val,
            None => panic!("Connect to client not established!")
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
        let client_options = ClientOptions::parse(db_uri).await.expect("There was an error parsing the DB_URI");
        
        let client = Client::with_options(client_options).expect("There was an error connecting to the database");
        
        println!("Connection to mongodb established!");
        
        self.client = Some(client);
    }

    async fn email_exists(&self, email: &String) -> Result<bool, ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<User>("users")
        .count_documents(doc!{ "email": email }, None).await {
            Ok(val) => Ok(val != 0),
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }

    async fn username_exists(&self, username: &String) -> Result<bool, ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<User>("users")
        .count_documents(doc!{ "username": username }, None).await {
            Ok(val) => Ok(val != 0),
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }

    async fn insert_user(&self, user: &User) -> Result<ObjectId, ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<User>("users")
        .insert_one(user, None).await {
            Ok(res) => {
                if let Some(obj_id) = res.inserted_id.as_object_id() {
                    Ok(obj_id)
                } else {
                    return Err(ApiErrors::ServerError("Error converting Object id".to_string()))
                }
            },
            Err(_) => Err(ApiErrors::ServerError(String::from("There was an issue storing the user")))
        }
    }

    async fn get_user(&self, username: &String) -> Result<User, ApiErrors>{
        match self.get_client()
        .database(&get_db_name())
        .collection::<User>("users")
        .find_one(doc!{ "username": username }, None).await {
            Ok(user) => {
                if let Some(a_user) = user {
                    Ok(a_user)
                } else {
                    Err(ApiErrors::NotFound("User not found".to_string()))
                }
            },
            Err(err) => Err(ApiErrors::BadRequest(err.to_string()))
        }
    }

    async fn get_all_user_records(&self, user_id: ObjectId) -> Result<Cursor<PasswordRecord>, ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<PasswordRecord>("records")
        .find(doc!{ "user_id": user_id }, None).await {
            Ok(res) => Ok(res),
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }

    async fn insert_record(&self, record: PasswordRecord) -> Result<ObjectId, ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<PasswordRecord>("records")
        .insert_one(record, None).await {
            Ok(res) => {
                if let Some(obj_id) = res.inserted_id.as_object_id() {
                    Ok(obj_id)
                } else {
                    return Err(ApiErrors::ServerError("Error converting Object id".to_string()))
                }
            },
            Err(error) => Err(ApiErrors::ServerError(error.to_string()))
        }
    }

    async fn get_record(
        &self, 
        record_id: ObjectId, 
        user_id: ObjectId
    ) -> Result<PasswordRecord, ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<PasswordRecord>("records")
        .find_one(doc!{ "_id": record_id, "user_id": user_id }, None).await {
            Ok(res) => {
                if let Some(record) = res {
                    Ok(record)
                } else {
                    Err(ApiErrors::NotFound("Record not found".to_string()))
                }
            },
            Err(error) => Err(ApiErrors::ServerError(error.to_string()))
        }
    }

    async fn delete_record(&self, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<PasswordRecord>("records")
        .find_one_and_delete(doc! {"_id": record_id, "user_id": user_id}, None).await {
            Ok(res) => {
                if res.is_none() {
                    return Err(ApiErrors::NotFound("Record not found".to_string()))
                }
                Ok(())
            },
            Err(error) => Err(ApiErrors::ServerError(error.to_string()))
        }
    }

    async fn update_record(
        &self,
        updated_record: UpdatePasswordRecord, 
        record_id: ObjectId, 
        user_id: ObjectId
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

        match self.get_client()
        .database(&get_db_name())
        .collection::<PasswordRecord>("records")
        .find_one_and_update(doc! { "_id": record_id, "user_id": user_id }, doc!{ "$set": update }, None).await {
            Ok(_) => Ok(()),
            Err(error) => Err(ApiErrors::ServerError(error.to_string()))
        }
    }

    async fn insert_secret(&self, secret_record: SecretRecord) -> Result<ObjectId, ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection("secrets")
        .insert_one(secret_record, None).await {
            Ok(res) => {
                match res.inserted_id.as_object_id() {
                    Some(id) => Ok(id),
                    None => Err(ApiErrors::ServerError("Error Converting ObjectId".to_string()))
                }
            },
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }

    async fn get_all_secret_records(&self, user_id: ObjectId) -> Result<Cursor<SecretRecord>, ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<SecretRecord>("secrets")
        .find(doc!{ "user_id": user_id }, None).await {
            Ok(res) => Ok(res),
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }

    async fn get_secret(&self, secret_id: ObjectId, user_id: ObjectId) -> Result<SecretRecord, ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<SecretRecord>("secrets")
        .find_one(doc! { "_id": secret_id, "user_id": user_id }, None).await {
            Ok(res) => {
                if let Some(secret) = res {
                    return Ok(secret)
                } else {
                    return Err(ApiErrors::NotFound("Secret Record Not Found.".to_string()))
                }
            },
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }

    async fn delete_secret(&self, secret_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
        match self.get_client()
        .database(&get_db_name())
        .collection::<SecretRecord>("secrets")
        .find_one_and_delete(doc! { "_id": secret_id, "user_id": user_id }, None).await {
            Ok(res) => {
                if res.is_none() {
                    return Err(ApiErrors::NotFound("Secret Record Not Found".to_string()))
                }
                Ok(())
            },
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }

    async fn update_secret(&self, updated_secret: UpdateSecretRecord, secret_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
        let mut updated_document = Document::new();

        if let Some(key) = updated_secret.key {
            updated_document.insert("key", key);
        }
        if let Some(secret) = updated_secret.secret {
            updated_document.insert("secret", secret);
        }

        match self.get_client()
        .database(&get_db_name())
        .collection::<SecretRecord>("secrets")
        .find_one_and_update(doc! { "_id": secret_id, "user_id": user_id }, updated_document, None).await {
            Ok(_) => Ok(()),
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }
}
