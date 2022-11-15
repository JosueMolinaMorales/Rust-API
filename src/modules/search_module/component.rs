use crate::shared::{types::ApiErrors, encryption::decrypt_password};

use super::*;


pub async fn search_records(
    db: &State<Box<dyn TMongoClient>>,
    search_params: SearchParams
) -> Result<Vec<SearchResponse>, ApiErrors> {
    // Inside search_params
    let mut res = db.search_records(search_params).await?;

    let mut record_vec = Vec::new();

    while let Some(record) = res.next().await {
        let mut record = record.map_err(|err| ApiErrors::ServerError(err.to_string()))?;
        
        match record.record_type {
            RecordTypes::Password => {
                if let Some(password) = record.password {
                    // Decrypt Password
                    record.password = Some(decrypt_password(&password)?);
                }
            },
            RecordTypes::Secret => {
                if let Some(secret) = record.secret {
                    // Decrypt Secret 
                    record.secret = Some(decrypt_password(&secret)?);
                }
            }
        }

        let record_id = record.id.ok_or(ApiErrors::ServerError("Object id was not found".to_string()))?.to_string();
        let user_id = record.user_id.ok_or(ApiErrors::ServerError("User id was not found in record".to_string()))?.to_string();
        // add to vector
        record_vec.push(SearchResponse::new(
            record_id,
            user_id,
            record.record_type,
            record.service,
            record.username,
            record.password,
            record.email,
            record.key,
            record.secret,
        ));
    }

    Ok(record_vec)
}