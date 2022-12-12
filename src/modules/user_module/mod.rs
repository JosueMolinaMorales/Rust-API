use bson::oid::ObjectId;
use rocket::{serde::json::Json, State, http::Status};

use crate::{drivers::mongodb::mongo_trait::TMongoClient, shared::{jwt_service::Token, types::{AuthUser, ApiErrors, UpdateUser}}};


#[get("/<user_id>")]
pub async fn get_user(
    db: &State<Box<dyn TMongoClient>>,
    user_id: String,
    user_token: Token
) -> Result<Json<AuthUser>, ApiErrors> {
    let user_id = ObjectId::parse_str(user_id)
        .map_err(|_| ApiErrors::BadRequest("User id is not a valid Object id".to_string()))?;

    if user_token.id.to_string() != user_id.to_string() {
        return Err(ApiErrors::Unauthorized("Not Authorized".to_string()))
    }

    let user = component::get_user(db, user_id).await?;

    Ok(Json(user))
}

#[patch("/<user_id>", data = "<updated_user>")]
pub async fn update_user(
    db: &State<Box<dyn TMongoClient>>,
    user_token: Token,
    user_id: String,
    updated_user: Json<UpdateUser>
) -> Result<Status, ApiErrors> {
    let user_id = ObjectId::parse_str(user_id)
    .map_err(|_| ApiErrors::BadRequest("User id is not a valid Object id".to_string()))?;

    if user_token.id.to_string() != user_id.to_string() {
        return Err(ApiErrors::Unauthorized("Not Authorized".to_string()))
    }

    component::update_user(db, user_id, updated_user.0).await?;

    Ok(Status::NoContent)
}



pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        get_user,
        update_user
    ]
}

pub mod component;