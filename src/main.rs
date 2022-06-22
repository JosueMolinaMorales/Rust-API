#[macro_use] extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use auth::auth_routehandler;
pub mod shared;
pub mod auth;
pub mod drivers;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let mut db = drivers::mongodb::MongoClient::new();
    db.connect().await;

    rocket::build()
    .manage(db)
    .mount("/", routes![index])
    .mount("/auth", routes![
        auth_routehandler::login,
        auth_routehandler::register,
    ])
}
