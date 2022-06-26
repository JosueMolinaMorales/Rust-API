use crate::auth::auth_datastore::AuthDatastore;
use crate::drivers::mongodb::MongoClient;
use crate::shared::types::{RegistrationForm, ApiErrors, User};

use super::rocket;
use rocket::State;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use mockall::*;
use mockall::predicate::*;

#[async_trait]
trait TAuthDatastore {
    fn build(db: &State<MongoClient>) -> AuthDatastore;
    async fn email_exists(&self, email: &String) -> Result<bool, ApiErrors>;
    async fn username_exists(&self, username: &String) -> Result<bool, ApiErrors>;
    async fn insert_user(&self, user: &User) -> Result<(), ApiErrors>;
    async fn get_user(&self, username: String) -> Result<Option<User>, ApiErrors>;
}

mock!{
    pub AuthDatastore {}
    #[async_trait]
    impl TAuthDatastore for AuthDatastore {
        fn build(db: &State<MongoClient>) -> AuthDatastore<'static>;
        async fn email_exists(&self, email: &String) -> Result<bool, ApiErrors>;
        async fn username_exists(&self, username: &String) -> Result<bool, ApiErrors>;
        async fn insert_user(&self, user: &User) -> Result<(), ApiErrors>;
        async fn get_user(&self, username: String) -> Result<Option<User>, ApiErrors>;
    }
}

#[rocket::async_test]
async fn register_success() {
    let req_body = RegistrationForm {
        email: "molinsa@dsa.com".to_string(),
        password: "Password".to_string(),
        firstname: "Josue".to_string(),
        lastname: "Morales".to_string(),
        username: "Testing123!".to_string()
    };
    let client = Client::tracked(super::rocket().await).await.unwrap();
    let req = client.post("/auth/register").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::Ok);
    let s1 = res.into_string().await;

    assert_eq!(s1.unwrap(), "User created!");
}
