#[macro_use]
extern crate rocket;
extern crate dotenv;
use dotenv::dotenv;
use crate::drivers::mongodb::mongo_trait::TMongoClient;
use modules::{auth_module, record_module, search_module};
pub mod drivers;
pub mod modules;
pub mod shared;

#[cfg(test)]
mod tests;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Password Manager API"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let mut db = drivers::mongodb::MongoClient::new();
    db.connect().await;

    rocket::build()
        .manage(Box::new(db) as Box<dyn TMongoClient>)
        .mount("/", routes![index])
        .mount("/auth/", auth_module::api())
        .mount("/search", search_module::api())
        .mount("/record", record_module::api())
}
