use crate::drivers::mongodb::{MockTMongoClient, TMongoClient};
use crate::{auth, password_manager};
use crate::shared::types::{RegistrationForm, ApiErrors, PasswordRecord, User, AuthResponse};
use bson::oid::ObjectId;
use dotenv::dotenv;
use self::mock_data::MockData;

use rocket::{Rocket, Build};
use rocket::http::Status;
use rocket::local::asynchronous::Client;
pub mod mock_data;

async fn mock_mongo_client(mock_data: &MockData) -> MockTMongoClient {
    let mut mock = MockTMongoClient::new();
    
    mock.expect_email_exists().returning(|email| {
        Ok(*email == "not_an_email@mail.gmail".to_string())
    });
    mock.expect_delete_record().returning(|record_id, user_id| {
        if record_id.to_string() == "notexist".to_string() || user_id.to_string() == "notexist".to_string() {
            return Err(ApiErrors::NotFound("Not Found".to_string()));
        }
        Ok(())
    });
    mock.expect_get_record().returning(|record_id, user_id| {
        Ok(Some(PasswordRecord {
        id: Some(record_id) ,
        service: "Netflix".to_string(),
            password: "Password".to_string(),
            email: Some("email@email.com".to_string()),
            username: Some("username".to_string()),
            user_id: Some(user_id),
        }))
    });
    mock.expect_get_user().returning(|_| {
        Ok(Some(User {
            id: Some(ObjectId::new()),
            name: "Name".to_string(),
            email: "email".to_string(),
            username: "username".to_string(),
            password : "password".to_string(),
        }))
    });
    mock.expect_insert_record().returning(|_| {
        Ok(ObjectId::new())
    });
    mock.expect_insert_user().returning(|_| {
        Ok(ObjectId::new())
    });
    mock.expect_update_record().returning(|_, record_id, user_id| {
        Ok(())
    });
    mock.expect_username_exists().returning(|username| {
        Ok(*username == "username_exist".to_string())
    });

    mock
}

async fn build_test_rocket(mock_data: &MockData) -> Rocket<Build> {
    dotenv().ok();

    let db = mock_mongo_client(mock_data).await;

    rocket::build()
    .manage(Box::new(db) as Box<dyn TMongoClient>)
    .mount("/auth", auth::api())
    .mount("/password/", password_manager::api())
}

/* Auth Tests */
#[rocket::async_test]
async fn register_success() {
    let req_body = RegistrationForm {
        email: "molinsa@dsa.com".to_string(),
        password: "Password".to_string(),
        name: "Josue Morales".to_string(),
        username: "Testing123!".to_string()
    };
    let mock_data = MockData::new();
    let client = Client::tracked(build_test_rocket(&mock_data).await).await.unwrap();
    let req = client.post("/auth/register").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::Ok);
    let s1 = res.into_json::<AuthResponse>().await.unwrap().user;
    
    assert_eq!(s1.email, req_body.email);
    assert_eq!(s1.name, req_body.name);
    assert_eq!(s1.username, req_body.username);
}

async fn register_fail_email_exist() {}

async fn register_fail_username_exist() {}

async fn login_success() {}

async fn login_fail_wrong_password() {}

async fn login_fail_wrong_username() {}

/* Password Manager Tests */

async fn create_record_success () {}

async fn update_record_success() {}

async fn update_record_fail_record_dne() {}

async fn update_record_fail_user_dne() {}

async fn delete_record_success() {}

async fn delete_record_fail_record_dne() {}

async fn delete_record_fail_user_dne() {}

async fn get_record_success() {}

async fn get_record_fail_record_dne() {}

async fn get_record_fail_user_dne() {}

