use crate::drivers::mongodb::mongo_trait::{MockTMongoClient, TMongoClient};
use crate::modules::{auth_module, record_module};
use crate::shared::types::{
    ApiErrors, AuthResponse, LoginForm, RegistrationForm,
    User, Record, UpdateRecord, RecordTypes,
};
use bson::doc;
use bson::oid::ObjectId;
use dotenv::dotenv;

use rocket::http::{Header, Status};
use rocket::local::asynchronous::Client;
use rocket::{Build, Rocket};

static DNE_OBJECTID: &str = "62e474fa9a8304a30105e2e0";
static AN_OBJECTID: &str = "62e489e380f15c93a32a7809";
static DNE_EMAIL: &str = "email_exists@mail.gmail";
static DNE_USERNANME: &str = "dne_username";
static USERNAME_EXISTS: &str = "username";
static EMAIL_EXISTS: &str = "email@email.com";
static HASH_PASSWORD: &str = "$2b$10$T2/jktu9x9fds7B0i/eoVeNMsFLLjwyeSsQMkrjxxKDcjaLo39g1y";
static PASSWORD: &str = "password123!";
static WRONG_PASSWORD: &str = "wrongPassword123!";
static BEARER_TOKEN: &str = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJpZCI6IjYyZTQ4OWUzODBmMTVjOTNhMzJhNzgwOSJ9.U-_fmdp_L1lxss_9q7s5WtWDpx6EmLdZ_YC5IH3dl90";
static BEARER_TOKEN_USER_DNE: &str = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJpZCI6IjYyZTQ3NGZhOWE4MzA0YTMwMTA1ZTJlMCJ9.YMHEIsGEwy6sI61OVgifyr1LU-GTsMtfWDFrh_DM9uU";
static NOT_VALID_TOKEN: &str = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";
static ENCRYPTED_PASSWORD: &str = "r7U7f8uiIzreDJMevRvx5g==";

async fn mock_mongo_client() -> MockTMongoClient {
    let mut mock = MockTMongoClient::new();

    mock.expect_email_exists()
        .returning(|email| Ok(*email == EMAIL_EXISTS.to_string()));

    mock.expect_delete_record().returning(|record_id, user_id| {
        if record_id.to_string() == DNE_OBJECTID.to_string()
            || user_id.to_string() == DNE_OBJECTID.to_string()
        {
            return Err(ApiErrors::NotFound("Not Found".to_string()));
        }
        Ok(())
    });

    mock.expect_get_record().returning(|record_id, user_id| {
        // Return error
        if record_id.to_string() == DNE_OBJECTID.to_string()
            || user_id.to_string() == DNE_OBJECTID.to_string()
        {
            return Err(ApiErrors::NotFound("Record not found".to_string()));
        }

        Ok(Record {
            id: Some(record_id),
            service: Some("Netflix".to_string()),
            password: Some(ENCRYPTED_PASSWORD.to_string()),
            email: Some("email@email.com".to_string()),
            username: Some("username".to_string()),
            user_id: Some(user_id),
            record_type: RecordTypes::Password,
            key: None,
            secret: None
        })
    });

    mock.expect_get_user().returning(|email| {
        if *email == DNE_EMAIL.to_string() {
            return Err(ApiErrors::BadRequest(
                "Username or Password is incorrect".to_string(),
            ));
        }
        Ok(User {
            id: Some(ObjectId::new()),
            name: "Name".to_string(),
            email: "email".to_string(),
            username: "username".to_string(),
            password: HASH_PASSWORD.to_string(),
        })
    });

    mock.expect_insert_record()
        .returning(|_| Ok(ObjectId::parse_str(AN_OBJECTID).unwrap()));
    mock.expect_insert_user().returning(|_| Ok(ObjectId::new()));
    mock.expect_update_record().returning(|_, _, _| Ok(()));
    mock.expect_username_exists()
        .returning(|username| Ok(*username == USERNAME_EXISTS.to_string()));

    mock
}

async fn build_test_rocket() -> Rocket<Build> {
    dotenv().ok();

    let db = mock_mongo_client().await;

    rocket::build()
        .manage(Box::new(db) as Box<dyn TMongoClient>)
        .mount("/auth", auth_module::api())
        .mount("/password/", record_module::api())
}

/* Auth Tests */
#[rocket::async_test]
async fn register_success() {
    let req_body = RegistrationForm {
        email: DNE_EMAIL.to_string(),
        password: "Password".to_string(),
        name: "Josue Morales".to_string(),
        username: DNE_USERNANME.clone().to_string(),
    };
    let client = Client::tracked(build_test_rocket().await).await.unwrap();
    let req = client.post("/auth/register").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::Ok);
    let s1 = res.into_json::<AuthResponse>().await.unwrap().user;

    assert_eq!(s1.email, req_body.email);
    assert_eq!(s1.name, req_body.name);
    assert_eq!(s1.username, req_body.username);
}

#[rocket::async_test]
async fn register_fail_email_exist() {
    let req_body = RegistrationForm {
        email: EMAIL_EXISTS.clone().to_string(),
        password: "password".to_string(),
        name: "Josue Morales".to_string(),
        username: DNE_USERNANME.clone().to_string(),
    };

    let client = Client::tracked(build_test_rocket().await).await.unwrap();
    let req = client.post("/auth/register").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::BadRequest);
}

#[rocket::async_test]
async fn register_fail_username_exist() {
    let req_body = RegistrationForm {
        email: DNE_EMAIL.clone().to_string(),
        password: "password".to_string(),
        name: "Josue Morales".to_string(),
        username: USERNAME_EXISTS.clone().to_string(),
    };

    let client = Client::tracked(build_test_rocket().await).await.unwrap();
    let req = client.post("/auth/register").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::BadRequest);
}

#[rocket::async_test]
async fn login_success() {
    let req_body = LoginForm {
        email: EMAIL_EXISTS.clone().to_string(),
        password: PASSWORD.to_string(),
    };

    let client = Client::tracked(build_test_rocket().await).await.unwrap();
    let req = client.post("/auth/login").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::Ok);
}

#[rocket::async_test]
async fn login_fail_wrong_password() {
    let req_body = LoginForm {
        email: EMAIL_EXISTS.clone().to_string(),
        password: WRONG_PASSWORD.to_string(),
    };

    let client = Client::tracked(build_test_rocket().await).await.unwrap();
    let req = client.post("/auth/login").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::BadRequest);
}

#[rocket::async_test]
async fn login_fail_wrong_username() {
    let req_body = LoginForm {
        email: DNE_EMAIL.clone().to_string(),
        password: WRONG_PASSWORD.to_string(),
    };

    let client = Client::tracked(build_test_rocket().await).await.unwrap();
    let req = client.post("/auth/login").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::BadRequest);
}

/* Authorization Errors */
#[rocket::async_test]
async fn no_authorization_header() {
    let req_body = doc! {
        "service": "Netflix".to_string(),
        "password": "password123!".to_string(),
        "email": "molinajosue92@test.com".to_string(),
    };
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client.post("/password").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::Unauthorized);
}

#[rocket::async_test]
async fn not_valid_auth_header() {
    let req_body = doc! {
        "service": "Netflix".to_string(),
        "password": "password123!".to_string(),
        "email": "molinajosue92@test.com".to_string(),
    };
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .post("/password")
        .json(&req_body)
        .header(Header::new("Authorization", NOT_VALID_TOKEN));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::Unauthorized);
}

/* Password Manager Tests */
#[rocket::async_test]
async fn create_record_success() {
    let req_body = doc! {
        "record_type": "Password",
        "service": "Netflix".to_string(),
        "password": "password123!".to_string(),
        "email": "molinajosue92@test.com".to_string(),
    };
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .post("/password")
        .json(&req_body)
        .header(Header::new("Authorization", BEARER_TOKEN));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::Created);
}

#[rocket::async_test]
async fn update_record_success() {
    let req_body = UpdateRecord {
        password: Some("new_password123".to_string()),
        email: Some("new_email@as.com".to_string()),
        username: None,
        service: None,
        key: None,
        secret: None
    };
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .patch(format!("/password/{}", AN_OBJECTID))
        .json(&req_body)
        .header(Header::new("Authorization", BEARER_TOKEN));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::NoContent);
}

#[rocket::async_test]
async fn update_record_fail_record_dne() {
    let req_body = UpdateRecord {
        password: Some("new_password123".to_string()),
        email: Some("new_email@as.com".to_string()),
        username: None,
        service: None,
        key: None,
        secret: None
    };
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .patch(format!("/password/{}", DNE_OBJECTID))
        .json(&req_body)
        .header(Header::new("Authorization", BEARER_TOKEN));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::NotFound);
}

#[rocket::async_test]
async fn update_record_fail_user_dne() {
    let req_body = UpdateRecord {
        password: Some("new_password123".to_string()),
        email: Some("new_email@as.com".to_string()),
        username: None,
        service: None,
        key: None,
        secret: None
    };
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .patch(format!("/password/{}", AN_OBJECTID))
        .json(&req_body)
        .header(Header::new("Authorization", BEARER_TOKEN_USER_DNE));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::NotFound);
}

#[rocket::async_test]
async fn delete_record_success() {
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .delete(format!("/password/{}", AN_OBJECTID))
        .header(Header::new("Authorization", BEARER_TOKEN));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::NoContent);
}

#[rocket::async_test]
async fn delete_record_fail_record_dne() {
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .delete(format!("/password/{}", DNE_OBJECTID))
        .header(Header::new("Authorization", BEARER_TOKEN));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::NotFound);
}

#[rocket::async_test]
async fn delete_record_fail_user_dne() {
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .delete(format!("/password/{}", AN_OBJECTID))
        .header(Header::new("Authorization", BEARER_TOKEN_USER_DNE));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::NotFound);
}

#[rocket::async_test]
async fn get_record_success() {
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .get(format!("/password/{}", AN_OBJECTID))
        .header(Header::new("Authorization", BEARER_TOKEN));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::Ok);
}

#[rocket::async_test]
async fn get_record_fail_record_dne() {
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .get(format!("/password/{}", DNE_OBJECTID))
        .header(Header::new("Authorization", BEARER_TOKEN));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::NotFound);
}

#[rocket::async_test]
async fn get_record_fail_user_dne() {
    let client = Client::tracked(build_test_rocket().await).await.unwrap();

    let req = client
        .get(format!("/password/{}", AN_OBJECTID))
        .header(Header::new("Authorization", BEARER_TOKEN_USER_DNE));

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::NotFound);
}
