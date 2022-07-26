#[macro_use] extern crate rocket;
extern crate dotenv;
use dotenv::dotenv;
use drivers::mongodb::TMongoClient;
pub mod shared;
pub mod auth;
pub mod drivers;
pub mod password_manager;

#[cfg(test)] mod tests;

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
    .manage(Box::new(db) as Box<dyn TMongoClient>)
    .mount("/", routes![index])
    .mount("/auth", auth::api())
    .mount("/password/", password_manager::api())
}
