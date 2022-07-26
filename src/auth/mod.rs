use rocket::serde::{Serialize, Deserialize};
use rocket::{State, serde::json::Json};

use crate::drivers::mongodb::TMongoClient;
use crate::shared::jwt_service::sign_token;
use crate::shared::types::{LoginForm, AuthUser, ApiErrors};
use crate::shared::types::RegistrationForm;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct AuthResponse {
    pub user: AuthUser,
    pub token: String
}

#[post("/login", data = "<login_form>")]
pub async fn login(db: &State<Box<dyn TMongoClient>>, login_form: Json<LoginForm>) -> Result<Json<AuthResponse>, ApiErrors> {
    let mut user = auth_component::login(db, login_form.0).await?;
    let id = user.id.clone().unwrap();
    user.id = None;
    let response = AuthResponse {
        user,
        token: sign_token(&id.to_string())?
    };
    Ok(Json(response))
}

#[post("/register", data = "<registration_form>")]
pub async fn register(
    db: &State<Box<dyn TMongoClient>>, 
    mut registration_form: Json<RegistrationForm>
) -> Result<Json<AuthResponse>, ApiErrors> {
    let mut user = auth_component::register(db, &mut registration_form.0).await?;
    let id = user.id.clone().unwrap();
    user.id = None;
    let response = AuthResponse {
        user,
        token: sign_token(&id.to_string()).unwrap()
    };
    Ok(Json(response))
    
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        login,
        register
    ]
}

pub mod auth_component;