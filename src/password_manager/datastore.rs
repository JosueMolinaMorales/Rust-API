use mongodb::{bson::doc, options::FindOneOptions};
use rocket::State;

use crate::{drivers::mongodb::MongoClient, shared::types::{PasswordRecord, ApiErrors, User, PartialUser}};

pub async fn insert_record(db: &State<MongoClient>, record: PasswordRecord) -> Result<(), ApiErrors> {
    match db.get_client()
    .database("personal-api")
    .collection::<PasswordRecord>("records")
    .insert_one(record, None).await {
        Ok(_) => { Ok(()) },
        Err(error) => {
            Err(ApiErrors::ServerError(error.to_string()))
        }
    }
}

pub async fn get_user_email(db: &State<MongoClient>, email: &String) -> Result<(), ApiErrors> {
    let options = FindOneOptions::builder()
    .projection(doc!{})
    match db.get_client()
    .database("personal-api")
    .collection::<User>("records")
    .find_one(doc!{ "email": email }, 
            FindOneOptions::builder().projection(doc!{ "email": 1 }).build()).await {
        Ok(res) => {
            match res {
                
            }
        },
        Err(err) => {

        }
    }
}