use crate::{shared::types::{ RegistrationForm, LoginForm, AuthUser, ApiErrors }, drivers::mongodb::MongoClient};
use mongodb::bson::doc;
use rocket::{serde::json::Json, State};
use validator::Validate;

use super::auth_component;


#[post("/login", data = "<login_form>")]
pub async fn login(db: &State<MongoClient>, login_form: Json<LoginForm>) -> Result<Json<AuthUser>, ApiErrors> {
    match auth_component::login(db, login_form.0).await {
        Ok(user) => {
            println!("{:?}", user);
            Ok(Json(user))
        },
        Err(err) => Err(err)
    }
}

#[post("/register", data = "<registration_form>")]
pub async fn register(db: &State<MongoClient>, mut registration_form: Json<RegistrationForm>) -> Result<Json<AuthUser>, ApiErrors> {
    match registration_form.0.validate() {
        Ok(_) => {},
        Err(_err) => {
            return Err(ApiErrors::BadRequest(String::from("Invalid Request")))
        }
    };
    match auth_component::register(db, &mut registration_form.0).await {
        Ok(auth_user) => Ok(Json(auth_user)),
        Err(err) => Err(err)
    }
}

