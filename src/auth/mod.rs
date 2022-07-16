use rocket::serde::{Serialize, Deserialize};
use rocket::{State, serde::json::Json};
use validator::Validate;

use crate::drivers::mongodb::MongoClient;
use crate::shared::jwt_service::sign_token;
use crate::shared::types::{LoginForm, AuthUser, ApiErrors};
use crate::shared::types::RegistrationForm;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct AuthResponse {
    user: AuthUser,
    token: String
}

#[post("/login", data = "<login_form>")]
pub async fn login(db: &State<MongoClient>, login_form: Json<LoginForm>) -> Result<Json<AuthResponse>, Json<ApiErrors>> {
    match auth_component::login(db, login_form.0).await {
        Ok(user) => {
            let id = user.id.clone().unwrap();
            let response = AuthResponse {
                user: AuthUser { id: None, name: user.name, email: user.email, username: user.username },
                token: sign_token(&id.to_string()).unwrap()
            };
            Ok(Json(response))
        },
        Err(err) => {
            Err(Json(err))
        }
    }
}

#[post("/register", data = "<registration_form>")]
pub async fn register(db: &State<MongoClient>, mut registration_form: Json<RegistrationForm>) -> Result<Json<AuthResponse>, ApiErrors> {
    match registration_form.0.validate() {
        Ok(_) => {},
        Err(_err) => {
            return Err(ApiErrors::BadRequest(String::from("Invalid Request")))
        }
    };
    match auth_component::register(db, &mut registration_form.0).await {
        Ok(user) => {
            let id = user.id.clone().unwrap();
            let response = AuthResponse {
                user: AuthUser { id: None, name: user.name, email: user.email, username: user.username },
                token: sign_token(&id.to_string()).unwrap()
            };
            Ok(Json(response))
        },
        Err(err) => Err(err)
    }
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        login,
        register
    ]
}

pub mod auth_component;
pub mod auth_datastore;