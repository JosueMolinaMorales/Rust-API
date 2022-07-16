use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rocket::State;

use crate::{shared::types::{PasswordRecord, ApiErrors}, drivers::mongodb::MongoClient, auth::auth_datastore};

use super::datastore;

pub(crate) async fn create_record(mut new_record: PasswordRecord, db: &State<MongoClient>) -> Result<(), ApiErrors> {
    // Validate email
    // let user = auth_datastore
    // Get ObjectId

    // Encrypt Password with Symmetric Key
    let mc = new_magic_crypt!("magickey", 256);
    new_record.password = mc.encrypt_to_base64(&new_record.password);

    // Store record in database
    match datastore::insert_record(db, new_record).await {
        Ok(_) => { Ok(()) },
        Err(err) => { Err(err) }
    }
}

pub fn update_record(record_id: String, db: &State<MongoClient>) {
    // Get the record

    // Check if the record exists

    // Update the record

    // Store the record
}

fn delete_record(record_id: String, db: &State<MongoClient>) {
    // Get the record

    // Check if the record exists

    // Delete record
}

fn get_record(record_id: String, db: &State<MongoClient>) {
    // Get the record

    // Check if the record exists

    // Return record
}