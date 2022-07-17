use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use mongodb::bson::oid::ObjectId;
use rocket::State;

use crate::{shared::types::{PasswordRecord, ApiErrors}, drivers::mongodb::MongoClient};

use super::datastore;

/**
 * Create a password record
 */
pub(crate) async fn create_record(db: &State<MongoClient>, mut new_record: PasswordRecord, id: ObjectId) -> Result<ObjectId, ApiErrors> {
    // Encrypt Password with Symmetric Key
    let mc = new_magic_crypt!("magickey", 256); // TODO: Make Key an env
    new_record.password = mc.encrypt_to_base64(&new_record.password);

    new_record.user_id = Some(id);
    // Store record in database
    match datastore::insert_record(db, new_record).await {
        Ok(id) => { Ok(id) },
        Err(err) => { Err(err) }
    }
}

pub async fn update_record(record_id: ObjectId, db: &State<MongoClient>) {
    // Get the record

    // Check if the record exists

    // Update the record

    // Store the record

}

pub async fn delete_record(db: &State<MongoClient>, record_id: String, user_id: ObjectId) -> Result<(), ApiErrors> {
    datastore::delete_record(db, ObjectId::parse_str(record_id).unwrap(), user_id).await?;
    Ok(())
}

pub async fn get_record( db: &State<MongoClient>, record_id: String, user_id: ObjectId) -> Result<PasswordRecord, ApiErrors>{
    // Get the record
    let record = datastore::get_record(
        db, 
        ObjectId::parse_str(record_id).unwrap(),
        user_id
    ).await?;

    // Check if the record exists and return
    match record {
        None => Err(ApiErrors::NotFound("Record was not found".to_string())),
        Some(rec) => {
            Ok(rec)
        }
    }
}