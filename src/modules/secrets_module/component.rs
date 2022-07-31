use bson::oid::ObjectId;
use rocket::State;

use crate::{drivers::mongodb::TMongoClient, shared::{types::{SecretRecord, ApiErrors, UpdateSecretRecord}, encryption::{encrypt_data, decrypt_password}}};

pub async fn create_secret(
    db: &State<Box<dyn TMongoClient>>,
    mut secret: SecretRecord
) -> Result<ObjectId, ApiErrors> {
    // Encrypt secret
    secret.secret = encrypt_data(&secret.secret);
    
    // Insert into Database
    let id = db.insert_secret(secret).await?;

    Ok(id)
}

pub async fn get_secret(
    db: &State<Box<dyn TMongoClient>>, 
    secret_id: ObjectId, 
    user_id: ObjectId
) -> Result<SecretRecord, ApiErrors> {

    let mut record = db.get_secret(secret_id, user_id).await?;

    // decrypt secret
    record.secret = decrypt_password(&record.secret)?;

    Ok(record)
}

pub async fn update_secret(
    db: &State<Box<dyn TMongoClient>>, 
    secret_id: ObjectId,
    user_id: ObjectId,
    updated_secret: UpdateSecretRecord
) -> Result<(), ApiErrors>{
    db.update_secret(updated_secret, secret_id, user_id).await?;
    Ok(())
}

pub async fn delete_secret(
    db: &State<Box<dyn TMongoClient>>, 
    secret_id: ObjectId, 
    user_id: ObjectId
) -> Result<(), ApiErrors> {
    db.delete_secret(secret_id, user_id).await?;

    Ok(())
}