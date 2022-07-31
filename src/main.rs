#[macro_use] extern crate rocket;
extern crate dotenv;
use dotenv::dotenv;
use drivers::mongodb::TMongoClient;
use modules::{ auth_module, password_module, secrets_module };
pub mod shared;
pub mod drivers;
pub mod modules;


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
    .mount("/auth/", auth_module::api())
    .mount("/password/", password_module::api())
    .mount("/secret/", secrets_module::api())
}
