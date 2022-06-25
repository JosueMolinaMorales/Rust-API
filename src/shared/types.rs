
use rocket::serde::{Deserialize, Serialize};

#[derive(Responder)]
pub enum ApiErrors {
    ServerError(String),
    ClientError(String),
    BadRequest(String),
    Forbidden(String)
}

impl ApiErrors {
    pub fn get_error(&self) -> String {
        match self {
            ApiErrors::BadRequest(err) => err.to_string(),
            ApiErrors::ClientError(err) => err.clone(),
            ApiErrors::Forbidden(err) => err.clone(),
            ApiErrors::ServerError(err) => err.clone()
        }
    }
}

#[derive(Deserialize)]
#[serde(crate="rocket::serde")]
#[derive(Debug)]
pub struct RegistrationForm {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate="rocket::serde")]
pub struct User {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Debug)]
pub struct AuthUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub username: String
}

#[derive(Deserialize)]
#[serde(crate="rocket::serde")]
#[derive(Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}
