#[macro_use] extern crate rocket;

pub mod auth;
pub mod drivers;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    connect_to_db();

    rocket::build()
    .mount("/", routes![index])
    .mount("/auth", routes![
        auth::login,
        auth::register,
        auth::get_account
    ])
}

async fn connect_to_db() {
    let client = drivers::mongodb::Mongo_client::new().connect().await;
}