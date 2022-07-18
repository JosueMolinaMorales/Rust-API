use mongodb::bson::{doc, oid::ObjectId};
use rocket::State;
use crate::{shared::types::{User, ApiErrors}, drivers::mongodb::MongoClient};

pub async fn email_exists(db: &State<MongoClient>, email: &String) -> Result<bool, ApiErrors> {
    match db.get_client()
    .database("personal-api")
    .collection::<User>("users")
    .count_documents(doc!{ "email": email }, None).await {
        Ok(val) => Ok(val != 0),
        Err(err) => Err(ApiErrors::ServerError(err.to_string()))
    }
}

pub async fn username_exists(db: &State<MongoClient>, username: &String) -> Result<bool, ApiErrors> {
    match db.get_client()
    .database("personal-api")
    .collection::<User>("users")
    .count_documents(doc!{ "username": username }, None).await {
        Ok(val) => Ok(val != 0),
        Err(err) => Err(ApiErrors::ServerError(err.to_string()))
    }
}

pub async fn insert_user(db: &State<MongoClient>, user: &User) -> Result<ObjectId, ApiErrors> {
    match db.get_client()
    .database("personal-api")
    .collection::<User>("users")
    .insert_one(user, None).await {
        Ok(res) => Ok(res.inserted_id.as_object_id().unwrap()),
        Err(_) => Err(ApiErrors::ServerError(String::from("There was an issue storing the user")))
    }
}

pub async fn get_user(db: &State<MongoClient>, username: &String) -> Result<Option<User>, ApiErrors>{
    match db.get_client()
    .database("personal-api")
    .collection::<User>("users")
    .find_one(doc!{ "username": username }, None).await {
        Ok(user) => Ok(user),
        Err(err) => Err(ApiErrors::BadRequest(err.to_string()))
    }
}

