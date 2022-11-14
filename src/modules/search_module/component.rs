use crate::shared::{types::ApiErrors, encryption::decrypt_password};

use super::*;


pub async fn search_password_records(
    db: &State<Box<dyn TMongoClient>>,
    user_id: ObjectId,
    search_params: SearchParams
) -> Result<Vec<SearchResponse>, ApiErrors> {
    // Inside search_params
    let mut res = db.search_secrets(search_params).await?;

    let mut record_vec = Vec::new();

    while let Some(record) = res.next().await {
        let mut record = record.map_err(|err| ApiErrors::ServerError(err.to_string()))?;
        // Decrypt Password
        record.secret = decrypt_password(&record.secret)?;

        let record_id = record.id.ok_or(ApiErrors::ServerError("Object id was not found".to_string()))?.to_string();
        // add to vector
        record_vec.push(SearchResponse::new(
            record_id,
            user_id.to_string(),
            RecordTypes::Secret,
            None,
            None,
            None,
            None,
            Some(record.key),
            Some(record.secret),
        ));
    }

    Ok(record_vec)
}

pub async fn search_secret_records(
    db: &State<Box<dyn TMongoClient>>,
    search_params: SearchParams
) -> Result<Vec<SearchResponse>, ApiErrors> {
    let mut res = db.search_records(search_params).await?;

    let mut record_vec = Vec::new();
    
    while let Some(record) = res.next().await {
        let mut record = record.map_err(|err| ApiErrors::ServerError(err.to_string()))?;
        // Decrypt Password
        record.password = decrypt_password(&record.password)?;

        let id = record
            .id
            .ok_or_else(|| ApiErrors::ServerError("No Objectid".to_string()))?
            .to_string();

        let user_id = record
            .user_id
            .ok_or_else(|| ApiErrors::ServerError("No Objectid".to_string()))?
            .to_string();

        // add to vector
        record_vec.push(SearchResponse::new(
            id,
            user_id,
            RecordTypes::Password,
            Some(record.service),
            record.username,
            Some(record.password),
            record.email,
            None,
            None
        ));
    }

    Ok(record_vec)
}