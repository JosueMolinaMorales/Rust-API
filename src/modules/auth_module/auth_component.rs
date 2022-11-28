use crate::{
    shared::{
        jwt_service::sign_token,
        types::{ApiErrors, AuthResponse, AuthUser, LoginForm, RegistrationForm, User},
    }, drivers::mongodb::mongo_trait::TMongoClient,
};
use pwhash::bcrypt;
use rocket::State;

pub async fn register(
    db: &State<Box<dyn TMongoClient>>,
    register_form: &mut RegistrationForm,
) -> Result<AuthResponse, ApiErrors> {
    /*
        Check if email exists -> check if email exists -> hash password -> insert user
    */
    if db.email_exists(&register_form.email).await? {
        return Err(ApiErrors::BadRequest(String::from("Email already exists")));
    }
    if db.username_exists(&register_form.username).await? {
        return Err(ApiErrors::BadRequest(String::from(
            "Username already exists",
        )));
    }

    register_form.password = match bcrypt::hash(&register_form.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(ApiErrors::ServerError(String::from(
                "There was an error hashing the password",
            )))
        }
    };

    // Create User
    let user: User = User {
        id: None,
        name: String::from(&register_form.name),
        email: String::from(&register_form.email).to_lowercase(),
        username: String::from(&register_form.username).to_lowercase(),
        password: String::from(&register_form.password),
    };

    // Insert user
    let id = db.insert_user(&user).await?;

    Ok(AuthResponse {
        user: AuthUser {
            name: String::from(&register_form.name),
            email: String::from(&register_form.email),
            username: String::from(&register_form.username),
        },
        token: sign_token(&id.to_string())?,
    })
}

pub async fn login(
    db: &State<Box<dyn TMongoClient>>,
    info: LoginForm,
) -> Result<AuthResponse, ApiErrors> {
    let err_msg = String::from("Username or password is incorrect");

    // Check to see if user exists
    let user = match db.get_user(&info.email.to_lowercase()).await {
        Ok(user) => user,
        Err(_) => return Err(ApiErrors::BadRequest(err_msg)),
    };

    // Match Password
    if !bcrypt::verify(&info.password, &user.password) {
        return Err(ApiErrors::BadRequest(err_msg));
    }

    let id = match user.id {
        Some(id) => id,
        None => {
            return Err(ApiErrors::ServerError(
                "There was an issue with the id".to_string(),
            ))
        }
    };

    Ok(AuthResponse {
        user: AuthUser {
            email: user.email,
            name: user.name,
            username: user.username,
        },
        token: sign_token(&id.to_string())?,
    })
}
