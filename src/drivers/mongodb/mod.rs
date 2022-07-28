use std::env;

use bson::{oid::ObjectId, Document, doc};
use mongodb::{ Client, options::ClientOptions };

use crate::shared::types::{ApiErrors, User, PasswordRecord, UpdatePasswordRecord};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TMongoClient: Send + Sync {
    async fn connect(&mut self);
    async fn email_exists(&self, email: &String) -> Result<bool, ApiErrors>;
    async fn username_exists(&self, username: &String) -> Result<bool, ApiErrors>;
    async fn insert_user(&self, user: &User) -> Result<ObjectId, ApiErrors>;
    async fn get_user(&self, username: &String) -> Result<Option<User>, ApiErrors>;
    async fn insert_record(&self, record: PasswordRecord) -> Result<ObjectId, ApiErrors>;
    async fn get_record(&self, record_id: ObjectId, user_id: ObjectId) -> Result<Option<PasswordRecord>, ApiErrors>;
    async fn delete_record(&self, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors>;
    async fn update_record(
        &self,  
        updated_record: UpdatePasswordRecord, 
        record_id: ObjectId, 
        user_id: ObjectId
    ) -> Result<(), ApiErrors>;
    
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
        let db_uri = match env::var("MONGODB_URI") {
            Ok(res) => res,
            Err(_) => {
                panic!("MONGODB_URI Env Not Set!")
            }
        };
        let client_options = ClientOptions::parse(db_uri).await.expect("There was an error parsing the DB_URI");
        
        let client = Client::with_options(client_options).expect("There was an error connecting to the database");
        
        println!("Connection to mongodb established!");
        
        self.client = Some(client);
    }

    async fn email_exists(&self, email: &String) -> Result<bool, ApiErrors> {
        match self.get_client()
        .database("personal-api")
        .collection::<User>("users")
        .count_documents(doc!{ "email": email }, None).await {
            Ok(val) => Ok(val != 0),
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }

    async fn username_exists(&self, username: &String) -> Result<bool, ApiErrors> {
        match self.get_client()
        .database("personal-api")
        .collection::<User>("users")
        .count_documents(doc!{ "username": username }, None).await {
            Ok(val) => Ok(val != 0),
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }

    async fn insert_user(&self, user: &User) -> Result<ObjectId, ApiErrors> {
        match self.get_client()
        .database("personal-api")
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

    async fn get_user(&self, username: &String) -> Result<Option<User>, ApiErrors>{
        match self.get_client()
        .database("personal-api")
        .collection::<User>("users")
        .find_one(doc!{ "username": username }, None).await {
            Ok(user) => Ok(user),
            Err(err) => Err(ApiErrors::BadRequest(err.to_string()))
        }
    }



    async fn insert_record(&self, record: PasswordRecord) -> Result<ObjectId, ApiErrors> {
        match self.get_client()
        .database("personal-api")
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
    ) -> Result<Option<PasswordRecord>, ApiErrors> {
        match self.get_client()
        .database("personal-api")
        .collection::<PasswordRecord>("records")
        .find_one(doc!{ "_id": record_id, "user_id": user_id }, None).await {
            Ok(res) => Ok(res),
            Err(error) => Err(ApiErrors::ServerError(error.to_string()))
        }
    }

    async fn delete_record(&self, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
        match self.get_client()
        .database("personal-api")
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
        .database("personal-api")
        .collection::<PasswordRecord>("records")
        .find_one_and_update(doc! { "_id": record_id, "user_id": user_id }, doc!{ "$set": update }, None).await {
            Ok(_) => Ok(()),
            Err(error) => Err(ApiErrors::ServerError(error.to_string()))
        }
    }
}
