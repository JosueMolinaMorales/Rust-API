pub mod component;
pub mod datastore;

use bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use rocket::State;
use rocket::http::Status;

use crate::drivers::mongodb::MongoClient;
use crate::shared::jwt_service::Token;
use crate::shared::types::{PasswordRecord, ApiErrors, UpdatePasswordRecord};
use rocket::serde::json::Json;

#[get("/<id>")]
pub async fn get_record(db: &State<MongoClient>, id: String, user_id: Token) -> Result<Json<PasswordRecord>, ApiErrors> {
    let record_id = match ObjectId::parse_str(id) {
        Ok(res) => res,
        Err(_) =>  return Err(ApiErrors::BadRequest("ID is not formatted correctly".to_string()))
    };
    match component::get_record(db, record_id, user_id.id).await {
        Ok(res) => {
            Ok(Json(res))
        },
        Err(error) => Err(error)
    }
}


#[post("/", data="<record>")]
pub async fn create_record(db: &State<MongoClient>, record: Json<PasswordRecord>, id: Token) -> Result<Json<Document>, ApiErrors> {
    match component::create_record(db, record.0, id.id).await {
        Ok(res) => Ok(Json(doc! { "id": res.to_string() })),
        Err(error) => Err(error)
    }
}

#[patch("/<id>", data="<updated_record>")]
pub async fn update_record(
    db: &State<MongoClient>, 
    updated_record: Json<UpdatePasswordRecord>, 
    id: String, 
    user_id: Token
) -> Result<Status, ApiErrors> {
    let record_id = match ObjectId::parse_str(id) {
        Ok(res) => res,
        Err(_) =>  return Err(ApiErrors::BadRequest("ID is not formatted correctly".to_string()))
    };
    component::update_record(
        db, 
        updated_record.0,
        record_id, 
        user_id.id).await?;
    Ok(Status::NoContent)
}

#[delete("/<id>")]
pub async fn delete_record(db: &State<MongoClient>, id: String, user_id: Token) -> Result<Status, ApiErrors> {
    let record_id = match ObjectId::parse_str(id) {
        Ok(res) => res,
        Err(_) =>  return Err(ApiErrors::BadRequest("ID is not formatted correctly".to_string()))
    };
    match component::delete_record(db, record_id, user_id.id).await {
        Ok(_) => Ok(Status::NoContent),
        Err(error) => Err(error)
    }
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        get_record,
        create_record,
        update_record,
        delete_record
    ]
}