pub mod component;
pub mod datastore;

use rocket::State;

use crate::drivers::mongodb::MongoClient;
use crate::shared::types::PasswordRecord;
use rocket::serde::json::Json;

#[get("/")]
pub async fn get_record(db: &State<MongoClient>) {
    todo!()
}

#[post("/", data="<record>")]
pub async fn create_record<'a>(db: &State<MongoClient>, record: Json<PasswordRecord>) -> &'a str {
    component::create_record(record.0, db).await;
    "Created!"
}

#[patch("/<id>")]
pub async fn update_record(db: &State<MongoClient>, id: String) {
    todo!()
}

#[delete("/<id>")]
pub async fn delete_record(db: &State<MongoClient>, id: String) {
    todo!()
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        get_record,
        create_record,
        update_record,
        delete_record
    ]
}