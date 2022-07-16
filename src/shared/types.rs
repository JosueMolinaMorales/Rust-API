use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};
use rocket::response::Responder;
use validator::Validate;

#[derive(Responder)]
#[derive(Serialize)]
#[serde(crate="rocket::serde")]
#[derive(Debug)]
pub enum ApiErrors {
    #[response(status = 500)]
    ServerError(String),

    #[response(status = 400)]
    BadRequest(String),

    #[response(status = 403)]
    Forbidden(String),

    #[response(status = 401)]
    Unauthorized(String)
}

impl ApiErrors {
    pub fn get_error(&self) -> String {
        match self {
            ApiErrors::BadRequest(err) => err.to_string(),
            ApiErrors::Unauthorized(err) => err.clone(),
            ApiErrors::Forbidden(err) => err.clone(),
            ApiErrors::ServerError(err) => err.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(crate="rocket::serde")]
pub struct RegistrationForm {
    pub name: String,

    #[validate(email)]
    pub email: String,
    pub username: String,
    pub password: String
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate="rocket::serde")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String
}

pub struct PartialUser {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct AuthUser {
    #[serde(rename = "_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub username: String
}

#[derive(Debug, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct LoginForm {
    pub username: String,
    pub password: String
}

/* Password Structs */
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct PasswordRecord {
    pub service: String, /* The Service the password belongs to */
    pub password: String, /* The Password for the service */
    pub email: Option<String>, /* The email to login */
    pub username: Option<String>,
    pub user_id: Option<String> /* The Object Id of the user who owns this record */
}