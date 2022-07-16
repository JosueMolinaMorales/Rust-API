use rocket::serde::{Deserialize, Serialize};
use rocket::response::Responder;
use validator::Validate;

#[derive(Responder)]
#[derive(Serialize)]
#[serde(crate="rocket::serde")]
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
    pub email: Option<String>, /* The email to login */
    pub username: Option<String>,
    pub password: String
}