use crate::{shared::types::{ RegistrationForm, LoginForm, AuthUser, ApiErrors }, drivers::mongodb::MongoClient};
use mongodb::bson::doc;
use rocket::{serde::json::Json, State};
use validator::Validate;

use super::auth_component::AuthComponent;


#[post("/login", data = "<login_form>")]
pub async fn login(db: &State<MongoClient>, login_form: Json<LoginForm>) -> Result<Json<AuthUser>, ApiErrors> {
    let auth = AuthComponent::build(db);
    match auth.login(login_form.0).await {
        Ok(user) => {
            println!("{:?}", user);
            Ok(Json(user))
        },
        Err(err) => Err(err)
    }
}

#[post("/register", data = "<registration_form>")]
pub async fn register(db: &State<MongoClient>, mut registration_form: Json<RegistrationForm>) -> Result<&str, ApiErrors> {
    let auth = AuthComponent::build(db);
    match registration_form.0.validate() {
        Ok(_) => {},
        Err(_err) => {
            return Err(ApiErrors::BadRequest(String::from("Invalid Request")))
        }
    };
    match auth.register(&mut registration_form.0).await {
        Ok(_) => Ok("User created!"),
        Err(err) => Err(err)
    }
}

