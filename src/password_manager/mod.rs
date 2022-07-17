pub mod component;
pub mod datastore;

use mongodb::bson::{doc, Document};
use rocket::State;
use rocket::http::Status;

use crate::drivers::mongodb::MongoClient;
use crate::shared::jwt_service::Token;
use crate::shared::types::{PasswordRecord, ApiErrors};
use rocket::serde::json::Json;

#[get("/<id>")]
pub async fn get_record(db: &State<MongoClient>, id: String, user_id: Token) -> Result<Json<PasswordRecord>, ApiErrors> {
    match component::get_record(db, id, user_id.id).await {
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

#[patch("/<id>")]
pub async fn update_record(db: &State<MongoClient>, id: String) {
    todo!()
}

#[delete("/<id>")]
pub async fn delete_record(db: &State<MongoClient>, id: String, user_id: Token) -> Result<Status, ApiErrors> {
    match component::delete_record(db, id, user_id.id).await {
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