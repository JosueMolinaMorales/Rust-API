use crate::{
    drivers::mongodb::mongo_trait::TMongoClient,
    shared::{
        encryption::{decrypt_password, encrypt_data},
        types::{ApiErrors, Record, ResponseRecord, RecordTypes, UpdateRecord},
    },
};
use mongodb::bson::oid::ObjectId;
use rocket::{futures::stream::StreamExt, State};

/**
 * Create a password record
 */
pub(crate) async fn create_record(
    db: &State<Box<dyn TMongoClient>>,
    mut new_record: Record,
    id: ObjectId,
) -> Result<ObjectId, ApiErrors> {
    match new_record.record_type {
        RecordTypes::Password => {
            if new_record.password.is_none() || new_record.service.is_none() {
                return Err(ApiErrors::BadRequest("Password and Service are required for a password record".to_string()));
            }
            // Password is being created validate that either username or email is provided
            if new_record.email.is_none() && new_record.username.is_none() {
                return Err(ApiErrors::BadRequest("Email or username is required for a password record".to_string()));
            }
            // Validate key and secret have not been passed in
            if new_record.key.is_some() || new_record.secret.is_some() {
                return Err(ApiErrors::BadRequest("Cannot create a secret and password record at the same time".to_string()));
            }
            if let Some(password) = new_record.password {
                new_record.password = Some(encrypt_data(&password));
            }
        },
        RecordTypes::Secret => {
            // Secret is being created, validate that a key is provided
            if new_record.key.is_none() {
                return Err(ApiErrors::BadRequest("Secret record requires a key".to_string()));
            }
            // Validate password record info was not passed in
            if new_record.email.is_some() || new_record.password.is_some() || new_record.username.is_some() {
                return Err(ApiErrors::BadRequest("Cannot create a secret and password record at the same time".to_string()));
            }
            if let Some(secret) = new_record.secret {
                new_record.secret = Some(encrypt_data(&secret));
            } else {
                return Err(ApiErrors::BadRequest("Secret is required for a secret record".to_string()));
            }
        }
    }

    new_record.user_id = Some(id);

    // Store record in database
    let record_id = db.insert_record(new_record).await?;
    Ok(record_id)
}

pub async fn update_record(
    db: &State<Box<dyn TMongoClient>>,
    mut updated_record: UpdateRecord,
    record_id: ObjectId,
    user_id: ObjectId,
) -> Result<(), ApiErrors> {
    // Get the record && Check if it exists
    let record = db.get_record(record_id, user_id).await?;

    match record.record_type {
        RecordTypes::Password => {
            // Validate no key/secret related items are attempting to be updated
            if updated_record.key.is_some() || updated_record.secret.is_some() {
                return Err(ApiErrors::BadRequest("Record is a password record, cannot update secret fields".to_string()));
            }
            if let Some(password) = updated_record.password {
                updated_record.password = Some(encrypt_data(&password));
            }
        },
        RecordTypes::Secret => {
            if 
                updated_record.email.is_some() || 
                updated_record.password.is_some() || 
                updated_record.service.is_some() || 
                updated_record.username.is_some()
            {
                return Err(ApiErrors::BadRequest("Record is secret record, cannot update password fields".to_string()));        
            }
            if let Some(secret) = updated_record.secret {
                updated_record.secret = Some(encrypt_data(&secret));
            }
        }
    }

    db.update_record(updated_record, record_id, user_id).await?;

    Ok(())
}

pub async fn delete_record(
    db: &State<Box<dyn TMongoClient>>,
    record_id: ObjectId,
    user_id: ObjectId,
) -> Result<(), ApiErrors> {
    db.delete_record(record_id, user_id).await?;
    Ok(())
}

pub async fn get_record(
    db: &State<Box<dyn TMongoClient>>,
    record_id: ObjectId,
    user_id: ObjectId,
) -> Result<ResponseRecord, ApiErrors> {
    // Get the record
    let mut record = db.get_record(record_id, user_id).await?;

    // Decrypt password
    if let Some(password) = record.password {
        record.password = Some(decrypt_password(&password)?);
    }
    if let Some(secret) = record.secret {
        record.secret = Some(decrypt_password(&secret)?);
    }

    let user_id = Some(record.user_id.ok_or(ApiErrors::ServerError("User id was not in record".to_string()))?.to_string());
    let id = Some(record.id.ok_or(ApiErrors::ServerError("Object id was not found for record".to_string()))?.to_string());

    Ok(ResponseRecord {
        record_type: record.record_type,
        user_id,
        id,
        key: record.key,
        secret: record.secret,
        service: record.service,
        password: record.password,
        email: record.email,
        username: record.username
    })
}

pub async fn get_all_user_records(
    db: &State<Box<dyn TMongoClient>>,
    user_id: ObjectId,
) -> Result<Vec<ResponseRecord>, ApiErrors> {
    let mut cursor = db.get_all_user_records(user_id).await?;
    let mut records: Vec<ResponseRecord> = Vec::new();

    while let Some(record) = cursor.next().await {
        let mut record = record.map_err(|err| ApiErrors::ServerError(err.to_string()))?;
        // Decrypt Password or secret
        if let Some(password) = record.password {
            // Record is a password
            record.record_type = RecordTypes::Password;
            record.password = Some(decrypt_password(&password)?);
        }
        if let Some(secret) = record.secret {
            // Record is secret
            record.record_type = RecordTypes::Secret;
            record.secret = Some(decrypt_password(&secret)?);
        }

        let id = match record.id {
            Some(id) => id.to_string(),
            None => return Err(ApiErrors::ServerError("No ObjectId".to_string())),
        };
        let user_id = match record.user_id {
            Some(id) => id.to_string(),
            None => return Err(ApiErrors::ServerError("No ObjectId".to_string())),
        };

        // add to vector
        records.push(ResponseRecord {
            id: Some(id),
            record_type: record.record_type,
            service: record.service,
            password: record.password,
            email: record.email,
            username: record.username,
            user_id: Some(user_id),
            key: record.key,
            secret: record.secret
        });
    }

    Ok(records)
}
