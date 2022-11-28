pub mod component;

use crate::{
    drivers::mongodb::mongo_trait::TMongoClient,
    shared::{
        jwt_service::Token,
        types::{
            ApiErrors, CreatedResponse,
            Record, ResponseRecord, UpdateRecord,
        },
    },
};
use bson::oid::ObjectId;
use mongodb::bson::doc;
use rocket::{http::Status, serde::json::Json, State};

/*
    Routes in this file:
    get /password/:id -> Get a password record
    POST /password -> Create a password record
    PATCH password/:id -> Update a password record
    DELETE /password/:id -> Delete a password record
*/

#[get("/<user_id>/all")]
pub async fn get_all_user_records(
    db: &State<Box<dyn TMongoClient>>,
    user_id: String,
    token: Token,
) -> Result<Json<Vec<ResponseRecord>>, ApiErrors> {
    let user_id = match ObjectId::parse_str(user_id) {
        Ok(res) => res,
        Err(_) => {
            return Err(ApiErrors::BadRequest(
                "User Id is not formatted correctly".to_string(),
            ))
        }
    };
    if token.id != user_id {
        return Err(ApiErrors::Unauthorized("Not Authorized".to_string()));
    }
    let records = component::get_all_user_records(db, user_id).await?;
    Ok(Json(records))
}

#[get("/<id>")]
pub async fn get_record(
    db: &State<Box<dyn TMongoClient>>,
    id: String,
    user_id: Token,
) -> Result<Json<ResponseRecord>, ApiErrors> {
    let record_id = match ObjectId::parse_str(id) {
        Ok(res) => res,
        Err(_) => {
            return Err(ApiErrors::BadRequest(
                "ID is not formatted correctly".to_string(),
            ))
        }
    };

    let res = component::get_record(db, record_id, user_id.id).await?;

    Ok(Json(res))
}

#[post("/", data = "<record>")]
pub async fn create_record(
    db: &State<Box<dyn TMongoClient>>,
    record: Json<Record>,
    id: Token,
) -> Result<CreatedResponse, ApiErrors> {
    let res = component::create_record(db, record.0, id.id).await?;
    Ok(CreatedResponse {
        id: Json(doc! { "id": res.to_string() }),
    })
}

#[patch("/<id>", data = "<updated_record>")]
pub async fn update_record(
    db: &State<Box<dyn TMongoClient>>,
    updated_record: Json<UpdateRecord>,
    id: String,
    user_id: Token,
) -> Result<Status, ApiErrors> {
    let record_id = match ObjectId::parse_str(id) {
        Ok(res) => res,
        Err(_) => {
            return Err(ApiErrors::BadRequest(
                "ID is not formatted correctly".to_string(),
            ))
        }
    };
    component::update_record(db, updated_record.0, record_id, user_id.id).await?;
    Ok(Status::NoContent)
}

#[delete("/<id>")]
pub async fn delete_record(
    db: &State<Box<dyn TMongoClient>>,
    id: String,
    user_id: Token,
) -> Result<Status, ApiErrors> {
    let record_id = match ObjectId::parse_str(id) {
        Ok(res) => res,
        Err(_) => {
            return Err(ApiErrors::BadRequest(
                "ID is not formatted correctly".to_string(),
            ))
        }
    };
    component::delete_record(db, record_id, user_id.id).await?;

    Ok(Status::NoContent)
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        get_record,
        create_record,
        update_record,
        delete_record,
        get_all_user_records
    ]
}
