use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use mongodb::bson::oid::ObjectId;
use rocket::State;

use crate::{shared::types::{PasswordRecord, ApiErrors}, drivers::mongodb::MongoClient};

use super::datastore;

pub(crate) async fn create_record(db: &State<MongoClient>, mut new_record: PasswordRecord, id: ObjectId) -> Result<(), ApiErrors> {
    // Validate email
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