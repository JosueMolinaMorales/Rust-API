use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use mongodb::bson::oid::ObjectId;
use rocket::State;

use crate::{shared::types::{PasswordRecord, ApiErrors, UpdatePasswordRecord}, drivers::mongodb::TMongoClient};


/**
 * Create a password record
 */
pub(crate) async fn create_record(db: &State<Box<dyn TMongoClient>>, mut new_record: PasswordRecord, id: ObjectId) -> Result<ObjectId, ApiErrors> {
    new_record.password = encrypt_password(&new_record.password);
    new_record.user_id = Some(id);
    // Store record in database
    let record_id = db.insert_record(new_record).await?;
    Ok(record_id)
}

pub async fn update_record(db: &State<Box<dyn TMongoClient>>, mut updated_record: UpdatePasswordRecord, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
    // Get the record
    let record = db.get_record(record_id, user_id).await?;
    // Check if the record exists
    if record.is_none() {
        return Err(ApiErrors::NotFound("Record not found".to_string()));
    }
    // Update the record
    if let Some(password) = updated_record.password {
        updated_record.password = Some(encrypt_password(&password));
    }
    db.update_record(updated_record, record_id, user_id).await?;

    Ok(())

}

pub async fn delete_record(db: &State<Box<dyn TMongoClient>>, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
    db.delete_record(record_id, user_id).await?;
    Ok(())
}

pub async fn get_record( db: &State<Box<dyn TMongoClient>>, record_id: ObjectId, user_id: ObjectId) -> Result<PasswordRecord, ApiErrors>{
    // Get the record
    let record = db.get_record(
        record_id,
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

fn encrypt_password(password: &String) -> String{
    // Encrypt Password with Symmetric Key
    let mc = new_magic_crypt!("magickey", 256); // TODO: Make Key an env
    mc.encrypt_to_base64(password)
}