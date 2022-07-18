use crate::{shared::types::{RegistrationForm, User, ApiErrors, LoginForm, AuthUser}, drivers::mongodb::MongoClient};
use pwhash::bcrypt;
use rocket::State;

use super::auth_datastore;

pub async fn register(db: &State<MongoClient>, register_form: &mut RegistrationForm) -> Result<AuthUser, ApiErrors> {
    /*
        Check if email exists -> check if email exists -> hash password -> insert user
    */
    if auth_datastore::email_exists(db, &register_form.email).await? {
        return Err(ApiErrors::BadRequest(String::from("Email already exists")));
    }
    if auth_datastore::username_exists(db, &register_form.username).await? {
        return Err(ApiErrors::BadRequest(String::from("Username already exists")));
    }

    let hash_pwd: String = match bcrypt::hash(&register_form.password) {
        Ok(hash) => hash,
        Err(_) => return Err(ApiErrors::ServerError(String::from("There was an error hashing the password")))
        
    };
    register_form.password = hash_pwd;

    let user: User = User {
        id: None,
        name: String::from(&register_form.name),
        email: String::from(&register_form.email),
        username: String::from(&register_form.username),
        password: String::from(&register_form.password)
    };

    let id = auth_datastore::insert_user(db, &user).await?;

    let auth_user = AuthUser {
        id: Some(id),
        name: String::from(&register_form.name),
        email: String::from(&register_form.email),
        username: String::from(&register_form.username),
    };

    Ok(auth_user)
}

pub async fn login(db: &State<MongoClient>, info: LoginForm) -> Result<AuthUser, ApiErrors> {
    let user: User;
    let err_msg = String::from("Username or password is incorrect");

    let res = auth_datastore::get_user(db, &info.username).await?;
    if res.is_none() {
        return Err(ApiErrors::BadRequest(err_msg))
    }
    user = res.unwrap();

    // Match Password
    if !bcrypt::verify(&info.password, &user.password) {
        return Err(ApiErrors::BadRequest(err_msg))
    }

    Ok(
        AuthUser {
            id: Some(user.id.clone().unwrap()),
            email: user.email,
            name: user.name,
            username: user.username
        }
    )
}


