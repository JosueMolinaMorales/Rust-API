
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug)]
enum Error {
    ServiceError,
    ClientErorr,
    NotAuthorized,
    Forbidden,
}

#[derive(Deserialize)]
#[serde(crate="rocket::serde")]
#[derive(Debug)]
pub struct RegistrationForm<'r> {
    pub firstname: &'r str,
    pub lastname: &'r str,
    pub email: &'r str,
    pub username: &'r str,
    pub password: &'r str
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate="rocket::serde")]
pub struct User<'r> {
    pub firstname: &'r str,
    pub lastname: &'r str,
    pub email: &'r str,
    pub username: &'r str,
    pub password: &'r str
}
