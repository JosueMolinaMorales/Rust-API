use rocket::{State, serde::json::Json};
use crate::drivers::mongodb::TMongoClient;
use crate::shared::types::{LoginForm, ApiErrors, AuthResponse};
use crate::shared::types::RegistrationForm;

#[post("/login", data = "<login_form>")]
pub async fn login(
    db: &State<Box<dyn TMongoClient>>, 
    login_form: Json<LoginForm>
) -> Result<Json<AuthResponse>, ApiErrors> {
    let response = auth_component::login(db, login_form.0).await?;
    Ok(Json(response))
}

#[post("/register", data = "<registration_form>")]
pub async fn register(
    db: &State<Box<dyn TMongoClient>>, 
    mut registration_form: Json<RegistrationForm>
) -> Result<Json<AuthResponse>, ApiErrors> {
    let response = auth_component::register(db, &mut registration_form.0).await?;
    Ok(Json(response))
    
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        login,
        register
    ]
}

pub mod auth_component;