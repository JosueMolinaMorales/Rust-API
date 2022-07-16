use rocket::{State, serde::json::Json};
use validator::Validate;

use crate::drivers::mongodb::MongoClient;
use crate::shared::types::{LoginForm, AuthUser, ApiErrors};
use crate::shared::types::RegistrationForm;

pub mod auth_component;
pub mod auth_datastore;

#[post("/login", data = "<login_form>")]
pub async fn login(db: &State<MongoClient>, login_form: Json<LoginForm>) -> Result<Json<AuthUser>, Json<ApiErrors>> {
    match auth_component::login(db, login_form.0).await {
        Ok(user) => {
            println!("{:?}", user);
            Ok(Json(user))
        },
        Err(err) => {
            Err(Json(err))
        }
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

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        login,
        register
    ]
}
