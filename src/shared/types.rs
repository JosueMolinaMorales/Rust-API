
use rocket::serde::{Deserialize, Serialize};

#[derive(Responder)]
pub enum ApiErrors<'a> {
    ServerError(&'a str),
    ClientError(&'a str),
    BadRequest(&'a str),
    Forbidden(&'a str)
}

impl ApiErrors<'_> {
    pub fn get_error(&self) -> &str {
        self
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
