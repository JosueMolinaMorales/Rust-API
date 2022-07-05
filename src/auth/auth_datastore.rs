use mongodb::bson::doc;
use rocket::State;
use crate::{shared::types::{User, ApiErrors}, drivers::mongodb::MongoClient};

#[async_trait]
pub trait TAuthDatastore {
    async fn email_exists(&self, email: &String) -> Result<bool, ApiErrors>;
    async fn username_exists(&self, username: &String) -> Result<bool, ApiErrors>;
    async fn insert_user(&self, user: &User) -> Result<(), ApiErrors>;
    async fn get_user(&self, username: String) -> Result<Option<User>, ApiErrors>;
}

pub struct AuthDatastore<'r> {
    db: &'r State<MongoClient>
}

impl <'r> AuthDatastore<'r> {
    pub fn build(db: &State<MongoClient>) -> AuthDatastore {
        AuthDatastore { db }
    }
}

#[async_trait]
impl <'r> TAuthDatastore for AuthDatastore<'r> {

    async fn email_exists(&self, email: &String) -> Result<bool, ApiErrors> {
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
    
    async fn username_exists(&self, username: &String) -> Result<bool, ApiErrors> {
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
    
    async fn insert_user(&self, user: &User) -> Result<(), ApiErrors> {
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
    
    async fn get_user(&self, username: String) -> Result<Option<User>, ApiErrors>{
        match self.db.get_client()
        .database("personal-api")
        .collection("users")
        .find_one(doc!{ "username": username }, None).await {
            Ok(user) => Ok(user),
            Err(err) => Err(ApiErrors::ServerError(err.to_string()))
        }
    }
}
