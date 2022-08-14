use bson::oid::ObjectId;
use rocket::{futures::stream::StreamExt, State};

use crate::{
    drivers::mongodb::TMongoClient,
    shared::{
        encryption::{decrypt_password, encrypt_data},
        types::{ApiErrors, SecretRecord, UpdateSecretRecord},
    },
};

use super::SecretRecordResponse;

pub async fn create_secret(
    db: &State<Box<dyn TMongoClient>>,
    mut secret: SecretRecord,
) -> Result<ObjectId, ApiErrors> {
    // Encrypt secret
    secret.secret = encrypt_data(&secret.secret);

    // Insert into Database
    let id = db.insert_secret(secret).await?;

    Ok(id)
}

pub async fn get_all_secret_record(
    db: &State<Box<dyn TMongoClient>>,
    user_id: ObjectId,
) -> Result<Vec<SecretRecordResponse>, ApiErrors> {
    let mut cursor = db.get_all_secret_records(user_id).await?;
    let mut records: Vec<SecretRecordResponse> = Vec::new();

    while let Some(record) = cursor.next().await {
        let mut record = record.map_err(|err| ApiErrors::ServerError(err.to_string()))?;

        // Decrypt Record secret
        record.secret = decrypt_password(&record.secret)?;

        let id = match record.id {
            Some(id) => id.to_string(),
            None => return Err(ApiErrors::ServerError("No Object id".to_string())),
        };
        let user_id = match record.user_id {
            Some(id) => id.to_string(),
            None => return Err(ApiErrors::ServerError("No Object id".to_string())),
        };

        records.push(SecretRecordResponse {
            id,
            user_id,
            key: record.key,
            secret: record.secret,
        })
    }

    Ok(records)
}

pub async fn get_secret(
    db: &State<Box<dyn TMongoClient>>,
    secret_id: ObjectId,
    user_id: ObjectId,
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
    updated_secret: UpdateSecretRecord,
) -> Result<(), ApiErrors> {
    db.update_secret(updated_secret, secret_id, user_id).await?;
    Ok(())
}

pub async fn delete_secret(
    db: &State<Box<dyn TMongoClient>>,
    secret_id: ObjectId,
    user_id: ObjectId,
) -> Result<(), ApiErrors> {
    db.delete_secret(secret_id, user_id).await?;

    Ok(())
}
