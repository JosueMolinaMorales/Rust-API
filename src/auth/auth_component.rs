use crate::{shared::types::{RegistrationForm, User, ApiErrors, LoginForm, AuthUser}, drivers::mongodb::TMongoClient};
use pwhash::bcrypt;
use rocket::State;


pub async fn register(db: &State<Box<dyn TMongoClient>>, register_form: &mut RegistrationForm) -> Result<AuthUser, ApiErrors> {
    /*
        Check if email exists -> check if email exists -> hash password -> insert user
    */
    if db.email_exists(&register_form.email).await? {
        return Err(ApiErrors::BadRequest(String::from("Email already exists")));
    }
    if db.username_exists(&register_form.username).await? {
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
        email: String::from(&register_form.email).to_lowercase(),
        username: String::from(&register_form.username).to_lowercase(),
        password: String::from(&register_form.password)
    };

    let id = db.insert_user(&user).await?;

    let auth_user = AuthUser {
        id: Some(id),
        name: String::from(&register_form.name),
        email: String::from(&register_form.email),
        username: String::from(&register_form.username),
    };

    Ok(auth_user)
}

pub async fn login(db: &State<Box<dyn TMongoClient>>, info: LoginForm) -> Result<AuthUser, ApiErrors> {
    let user: User;
    let err_msg = String::from("Username or password is incorrect");

    let res = db.get_user(&info.username.to_lowercase()).await?;
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


