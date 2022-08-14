/*
    Secrets module will handle CRUD for secrets other than passwords
    Schema: {
        _id: Objectid,
        user_id: Objectid,
        key: string,
        secret: string
    }
*/
pub mod component;
use std::str::FromStr;

use crate::{
    drivers::mongodb::TMongoClient,
    shared::{
        jwt_service::Token,
        types::{ApiErrors, SecretRecord, UpdateSecretRecord},
    },
};
use bson::{doc, oid::ObjectId, Document};
use rocket::serde::json::Json;
use rocket::{http::Status, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SecretRecordResponse {
    pub id: String,
    pub user_id: String,
    pub key: String,
    pub secret: String,
}

#[get("/records/<user_id>")]
pub async fn get_all_secret_record(
    db: &State<Box<dyn TMongoClient>>,
    token: Token,
    user_id: String,
) -> Result<Json<Vec<SecretRecordResponse>>, ApiErrors> {
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
    let records = component::get_all_secret_record(db, user_id).await?;
    Ok(Json(records))
}

#[get("/<id>")]
pub async fn get_secret(
    db: &State<Box<dyn TMongoClient>>,
    id: String,
    user_id: Token,
) -> Result<Json<SecretRecordResponse>, ApiErrors> {
    // Validate Object id
    let secret_id = match ObjectId::from_str(&id) {
        Ok(id) => id,
        Err(err) => return Err(ApiErrors::BadRequest(err.to_string())),
    };
    let secret_record = component::get_secret(db, secret_id, user_id.id).await?;

    Ok(Json(SecretRecordResponse {
        id,
        user_id: user_id.id.to_string(),
        key: secret_record.key,
        secret: secret_record.secret,
    }))
}

#[delete("/<id>")]
pub async fn delete_secret(
    db: &State<Box<dyn TMongoClient>>,
    id: String,
    user_id: Token,
) -> Result<Status, ApiErrors> {
    let secret_id = match ObjectId::from_str(&id) {
        Ok(id) => id,
        Err(err) => return Err(ApiErrors::BadRequest(err.to_string())),
    };

    component::delete_secret(db, secret_id, user_id.id).await?;

    Ok(Status::NoContent)
}

#[post("/", data = "<secret>")]
pub async fn create_secret(
    db: &State<Box<dyn TMongoClient>>,
    mut secret: Json<SecretRecord>,
    user_id: Token,
) -> Result<Json<Document>, ApiErrors> {
    secret.0.user_id = Some(user_id.id);
    let id = component::create_secret(db, secret.0).await?;
    Ok(Json(doc! {
        "id": id.to_string()
    }))
}

#[patch("/<id>", data = "<updated_secret>")]
pub async fn update_secret(
    db: &State<Box<dyn TMongoClient>>,
    id: String,
    updated_secret: Json<UpdateSecretRecord>,
    user_id: Token,
) -> Result<Status, ApiErrors> {
    let secret_id = match ObjectId::from_str(&id) {
        Ok(id) => id,
        Err(err) => return Err(ApiErrors::BadRequest(err.to_string())),
    };
    component::update_secret(db, secret_id, user_id.id, updated_secret.0).await?;

    Ok(Status::NoContent)
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        get_secret,
        delete_secret,
        create_secret,
        update_secret,
        get_all_secret_record
    ]
}
