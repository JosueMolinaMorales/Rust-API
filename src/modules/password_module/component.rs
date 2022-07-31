use mongodb::bson::oid::ObjectId;
use rocket::{State, futures::stream::StreamExt};
use crate::{shared::{types::{PasswordRecord, ApiErrors, UpdatePasswordRecord, ResponsePasswordRecord}, encryption::{decrypt_password, encrypt_data}}, drivers::mongodb::TMongoClient};


/**
 * Create a password record
 */
pub(crate) async fn create_record(db: &State<Box<dyn TMongoClient>>, mut new_record: PasswordRecord, id: ObjectId) -> Result<ObjectId, ApiErrors> {
    new_record.password = encrypt_data(&new_record.password);
    new_record.user_id = Some(id);
    // Store record in database
    let record_id = db.insert_record(new_record).await?;
    Ok(record_id)
}

pub async fn update_record(db: &State<Box<dyn TMongoClient>>, mut updated_record: UpdatePasswordRecord, record_id: ObjectId, user_id: ObjectId) -> Result<(), ApiErrors> {
    // Get the record && Check if it exists
    db.get_record(record_id, user_id).await?;

    // Update the record
    if let Some(password) = updated_record.password {
        updated_record.password = Some(encrypt_data(&password));
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
    let mut record = db.get_record(
        record_id,
        user_id
    ).await?;

    // Decrypt password
    record.password = decrypt_password(&record.password)?;

    Ok(record)
}

pub async fn get_all_user_records(db: &State<Box<dyn TMongoClient>>, user_id: ObjectId) -> Result<Vec<ResponsePasswordRecord>, ApiErrors> {
    let mut cursor = db.get_all_user_records(user_id).await?;
    let mut records: Vec<ResponsePasswordRecord> = Vec::new();

    while let Some(record) = cursor.next().await {
        let mut record = record.map_err(|err| ApiErrors::ServerError(err.to_string()))?;
        // Decrypt Password
        record.password = decrypt_password(&record.password)?;

        // Make oid to string
        record.id = 
        // add to vector
        records.push(ResponsePasswordRecord { 
            id: (), 
            service: (), 
            password: (), 
            email: (), 
            username: (), 
            user_id: () 
        });
    }

    Ok(records)
}
