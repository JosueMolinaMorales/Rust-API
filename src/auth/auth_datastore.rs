use mongodb::bson::doc;
use rocket::State;
use crate::{shared::types::{User, ApiErrors}, drivers::mongodb::MongoClient};

pub struct AuthDatastore<'r> {
    db: &'r State<MongoClient>
}

impl <'r> AuthDatastore<'r> {
    pub fn build(db: &State<MongoClient>) -> AuthDatastore {
        AuthDatastore { db }
    }

    pub async fn email_exists(&self, email: &String) -> Result<bool, ApiErrors> {
        match self.db.get_client()
        .database("personal-api")
        .collection::<User>("users")
        .count_documents(doc!{ "email": email }, None).await {
            Ok(val) => { 
                Ok(val != 0)
            },
            Err(err) => {
                Err(ApiErrors::ServerError(err.to_string()))
            }
        }
    }
    
    pub async fn username_exists(&self, username: &String) -> Result<bool, ApiErrors> {
        match self.db.get_client()
        .database("personal-api")
        .collection::<User>("users")
        .count_documents(doc!{ "username": username }, None).await {
            Ok(val) => {
                Ok(val != 0)
            },
            Err(err) => {
                Err(ApiErrors::ServerError(err.to_string()))
            }
        }
    }
    
    pub async fn insert_user(&self, user: &User) -> Result<(), ApiErrors> {
        match self.db.get_client()
        .database("personal-api")
        .collection::<User>("users")
        .insert_one(user, None).await {
            Ok(_) => {Ok(())},
            Err(_) => {
                Err(ApiErrors::ServerError(String::from("There was an issue storing the user")))
            }
        }
    }
    
    pub async fn get_user(&self, username: String) -> Result<Option<User>, ApiErrors>{
        match self.db.get_client()
        .database("personal-api")
        .collection::<User>("users")
        .find_one(doc!{ "username": username }, None).await {
            Ok(user) => Ok(user),
            Err(err) => Err(ApiErrors::BadRequest(err.to_string()))
        }
    }
}
