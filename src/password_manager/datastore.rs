use bson::Document;
use mongodb::bson::{oid::ObjectId, doc};
use rocket::State;

use crate::{drivers::mongodb::MongoClient, shared::types::{PasswordRecord, ApiErrors, UpdatePasswordRecord}};

pub async fn insert_record(db: &State<MongoClient>, record: PasswordRecord) -> Result<ObjectId, ApiErrors> {
    match db.get_client()
    .database("personal-api")
    .collection::<PasswordRecord>("records")
    .insert_one(record, None).await {
        Ok(res) => { Ok(res.inserted_id.as_object_id().unwrap()) },
        Err(error) => {
            Err(ApiErrors::ServerError(error.to_string()))
        }
    }
}

pub async fn get_record(db: &State<MongoClient>, record_id: ObjectId, user_id: ObjectId) -> Result<Option<PasswordRecord>, ApiErrors> {
    match db.get_client()
    .database("personal-api")
    .collection::<PasswordRecord>("records")
    .find_one(doc!{ "_id": record_id, "user_id": user_id }, None).await {
        Ok(res) => Ok(res),
        Err(error) => Err(ApiErrors::ServerError(error.to_string()))
    }
}

pub async fn delete_record(db: &State<MongoClient>, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
    match db.get_client()
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

pub async fn update_record(
    db: &State<MongoClient>, 
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

    match db.get_client()
    .database("personal-api")
    .collection::<PasswordRecord>("records")
    .find_one_and_update(doc! { "_id": record_id, "user_id": user_id }, doc!{ "$set": update }, None).await {
        Ok(_) => Ok(()),
        Err(error) => Err(ApiErrors::ServerError(error.to_string()))
    }
}