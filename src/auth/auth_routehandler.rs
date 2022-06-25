use crate::{shared::types::{ RegistrationForm, LoginForm }, drivers::mongodb::MongoClient};
use mongodb::bson::doc;
use rocket::{serde::json::Json, State};

use super::auth_component::AuthComponent;


#[post("/login", data = "<login_form>")]
pub async fn login(db: &State<MongoClient>, login_form: Json<LoginForm>) -> Result<&str, String> {
    let auth = AuthComponent::build(db);
    match auth.login(login_form.0).await {
        Ok(user) => {
            println!("{:?}", user);
            Ok("Logged in!")
        },
        Err(err) => Err(err.get_error())
    }
}

#[post("/register", data = "<registration_form>")]
pub async fn register(db: &State<MongoClient>, mut registration_form: Json<RegistrationForm>) -> Result<&str, String> {
    let auth = AuthComponent::build(db);

    match auth.register(&mut registration_form.0).await {
        Ok(_) => Ok("User created!"),
        Err(err) => Err(err.get_error())
    }
}

