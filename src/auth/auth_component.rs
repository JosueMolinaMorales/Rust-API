use crate::{shared::types::{RegistrationForm, User, ApiErrors, LoginForm, AuthUser}, drivers::mongodb::MongoClient};
use pwhash::bcrypt;
use rocket::State;

use super::auth_datastore::AuthDatastore;

pub struct AuthComponent<'r> {
    datastore: AuthDatastore<'r>
}

impl <'r> AuthComponent<'r> {
    pub fn build(db: &State<MongoClient>) -> AuthComponent {
        AuthComponent {
            datastore: AuthDatastore::build(db)
        }
    }

    pub async fn register<'s>(&'s self, register_form: &mut RegistrationForm) -> Result<AuthUser, ApiErrors> {
        /*
            Check if email exists -> check if email exists -> hash password -> insert user
        */
        if self.datastore.email_exists(&register_form.email).await? {
            return Err(ApiErrors::BadRequest(String::from("Email already exists")));
        }
        if self.datastore.username_exists(&register_form.username).await? {
            return Err(ApiErrors::BadRequest(String::from("Username already exists")));
        }
    
        let hash_pwd: String = match bcrypt::hash(&register_form.password) {
            Ok(hash) => {
                hash
            },
            Err(_) => {
                return Err(ApiErrors::ServerError(String::from("There was an error hashing the password")));
            }
        };
        register_form.password = hash_pwd;
    
        let user: User = User {
            firstname: String::from(&register_form.firstname),
            lastname: String::from(&register_form.lastname),
            email: String::from(&register_form.email),
            username: String::from(&register_form.username),
            password: String::from(&register_form.password)
        };
    
        self.datastore.insert_user(&user).await?;

        let auth_user = AuthUser {
            firstname: String::from(&register_form.firstname),
            lastname: String::from(&register_form.lastname),
            email: String::from(&register_form.email),
            username: String::from(&register_form.username),
        };
    
        Ok(auth_user)
    }

    pub async fn login(&self, info: LoginForm) -> Result<AuthUser, ApiErrors> {
        let user: User;
        let err_msg = String::from("Username or password is incorrect");

        match self.datastore.get_user(info.username).await {
            Ok(res) => {
                if let Some(a_user) = res {
                    user = a_user
                } else {
                    return Err(ApiErrors::BadRequest(err_msg))
                }
            },
            Err(err) => return Err(err)
        }

        // Match Password
        if !bcrypt::verify(&info.password, &user.password) {
            return Err(ApiErrors::BadRequest(err_msg))
        }


        Ok(
            AuthUser {
                email: user.email,
                firstname: user.firstname,
                lastname: user.lastname,
                username: user.username
            }
        )
    }

}

