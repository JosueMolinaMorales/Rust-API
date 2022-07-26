pub mod component;

use bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use rocket::{ State, http::Status, serde::json::Json };
use crate::{ 
    drivers::mongodb::TMongoClient, 
    shared::{ jwt_service::Token, types::{PasswordRecord, ApiErrors, UpdatePasswordRecord, ResponsePasswordRecord} } 
};

/*
    Routes in this file:
    get /password/:id -> Get a password record
    POST /password -> Create a password record
    PATCH password/:id -> Update a password record
    DELETE /password/:id -> Delete a password record
*/

#[get("/<id>")]
pub async fn get_record(
    db: &State<Box<dyn TMongoClient>>, 
    id: String, 
    user_id: Token
) -> Result<Json<ResponsePasswordRecord>, ApiErrors> {
    let record_id = match ObjectId::parse_str(id) {
        Ok(res) => res,
        Err(_) =>  return Err(ApiErrors::BadRequest("ID is not formatted correctly".to_string()))
    };
    
    let res = component::get_record(db, record_id, user_id.id).await?;
    
    Ok(Json(ResponsePasswordRecord { 
        id: res.id.unwrap().to_string(), 
        service: res.service, 
        password: res.password, 
        email: res.email, 
        username: res.username,
        user_id: res.user_id.unwrap().to_string()
    }))
}


#[post("/", data="<record>")]
pub async fn create_record(
    db: &State<Box<dyn TMongoClient>>, 
    record: Json<PasswordRecord>, 
    id: Token
) -> Result<Json<Document>, ApiErrors> {
    let res = component::create_record(db, record.0, id.id).await?;
    Ok(Json(doc! {
        "id": res.to_string()
    }))
}

#[patch("/<id>", data="<updated_record>")]
pub async fn update_record(
    db: &State<Box<dyn TMongoClient>>, 
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
pub async fn delete_record(db: &State<Box<dyn TMongoClient>>, id: String, user_id: Token) -> Result<Status, ApiErrors> {
    let record_id = match ObjectId::parse_str(id) {
        Ok(res) => res,
        Err(_) =>  return Err(ApiErrors::BadRequest("ID is not formatted correctly".to_string()))
    };
    component::delete_record(db, record_id, user_id.id).await?; 

    Ok(Status::NoContent)
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        get_record,
        create_record,
        update_record,
        delete_record
    ]
}