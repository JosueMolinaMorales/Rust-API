use crate::{shared::types::{RegistrationForm, User, ApiErrors}, drivers::mongodb::MongoClient};

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

    pub async fn register<'s>(&'s self, register_form: &mut RegistrationForm) -> Result<User, ApiErrors<'s>> {
        /*
            Check if email exists -> check if email exists -> hash password -> insert user
        */
        if self.datastore.email_exists(&register_form.email) {
            return Err(ApiErrors::BadRequest("Email already exists"));
        }
        if self.datastore.username_exists(&register_form.username) {
            return Err(ApiErrors::BadRequest("Username already exists"));
        }
    
        let hash_pwd: String = match bcrypt::hash(&register_form.password) {
            Ok(hash) => {
                hash
            },
            Err(_) => {
                return Err(ApiErrors::ServerError("There was an error hashing the password"));
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
    
        Ok(user)
    }
}

