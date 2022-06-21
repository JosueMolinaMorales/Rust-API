use crate::{shared::types::{ RegistrationForm, User }, drivers::mongodb::Mongo_client};
use mongodb::bson::doc;
use rocket::{serde::json::Json, State};

#[post("/login")]
pub fn login() -> &'static str {
    "Login works!"
}

#[post("/register", data = "<registration_form>")]
pub async fn register(db: &State<Mongo_client>, registration_form: Json<RegistrationForm<'_>>) -> &'static str {
    let res = db.get_client().database("personal-api").collection("users").insert_one(User {
        firstname: registration_form.firstname,
        lastname: registration_form.lastname,
        email: registration_form.email,
        password: registration_form.password,
        username: registration_form.username,
    }, None).await;
    match res {
        Ok(val) => {
            println!("{:?}", val)
        },
        Err(_) => {}
    };
    println!("{:?}", registration_form);
    "Printed!"
}

#[get("/account/<acc>")]
pub fn get_account(acc: String) -> String {
    acc
}
