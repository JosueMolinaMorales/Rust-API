use crate::shared::types::RegistrationForm;

use super::rocket;
use rocket::http::Status;
use rocket::local::asynchronous::Client;


#[rocket::async_test]
async fn register_success() {
    let req_body = RegistrationForm {
        email: "molinsa@dsa.com".to_string(),
        password: "Password".to_string(),
        name: "Josue Morales".to_string(),
        username: "Testing123!".to_string()
    };
    let client = Client::tracked(super::rocket().await).await.unwrap();
    let req = client.post("/auth/register").json(&req_body);

    let res = req.dispatch().await;

    assert_eq!(res.status(), Status::Ok);
    let s1 = res.into_string().await;

    assert_eq!(s1.unwrap(), "User created!");
}
