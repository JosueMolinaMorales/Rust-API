#[macro_use] extern crate rocket;

mod auth;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/auth", routes![
        auth::login,
        auth::register,
        auth::get_account
    ])
}