use crate::{shared::types::{ RegistrationForm, ApiErrors }, drivers::mongodb::MongoClient};
use mongodb::bson::doc;
use rocket::{serde::json::Json, State};

use super::auth_component::AuthComponent;


#[post("/login")]
pub fn login() -> &'static str {
    "Login works!"
}

#[post("/register", data = "<registration_form>")]
pub async fn register(db: &State<MongoClient>, mut registration_form: Json<RegistrationForm>) -> &str {
    let auth = AuthComponent::build(db);

    match auth.register(&mut registration_form.0).await {
        Ok(val) => "User created!",
        Err(err) => "There was an error"
    }
}

