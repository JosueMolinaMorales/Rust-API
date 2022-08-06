/*
    Searching Module will be used for searching through password records and secret records
    /<user_id>/search?record=&page=&service=&key=
    * Record will be either secret or password
    * Page will be the pagination number
    * Service will be the name of the service to search for
    * Key will be the key of the secret to get
*/
use rocket::{futures::stream::StreamExt, serde::json::Json};

#[derive(Debug, PartialEq, FromFormField)]
pub enum RecordTypes {
    Password,
    Secret
}

pub struct SearchResponse {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,

    key: Option<String>,
    secret: Option<String>,
}

pub struct SearchParamsBuilder {
    pub user_id: ObjectId,
    pub password_record: Option<RecordTypes>,
    pub secret_record: Option<RecordTypes>,
    pub page: Option<u64>,
    pub service: Option<String>,
    pub key: Option<String>,
    pub limit: Option<i64>,
}

impl SearchParamsBuilder {
    pub fn new(user_id: ObjectId) -> SearchParamsBuilder {
        SearchParamsBuilder { 
            user_id, 
            password_record: None,
            secret_record: None, 
            page: None, 
            service: None, 
            key: None, 
            limit: None
        }
    }

    pub fn add_record(mut self, record_type: RecordTypes) -> Self {
        match record_type {
            RecordTypes::Password => self.password_record = Some(RecordTypes::Password),
            RecordTypes::Secret => self.secret_record = Some(RecordTypes::Secret)
        };
        self
    }

    pub fn add_page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }

    pub fn add_service(mut self, service: String) -> Self {
        self.service = Some(service);
        self
    }

    pub fn add_key(mut self, key: String) -> Self {
        self.key = Some(key);
        self
    }

    pub fn add_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn build(self) -> SearchParams {
        SearchParams { 
            user_id: self.user_id, 
            password_record: self.password_record, 
            secret_record: self.secret_record, 
            page: self.page, 
            service: self.service, 
            key: self.key, 
            limit: self.limit
        }
    }
}

#[derive(Debug)]
pub struct SearchParams {
    pub user_id: ObjectId,
    pub password_record: Option<RecordTypes>,
    pub secret_record: Option<RecordTypes>, 
    pub page: Option<u64>,
    pub service: Option<String>,
    pub key: Option<String>,
    pub limit: Option<i64>,
}

use bson::oid::ObjectId;
use rocket::State;

use crate::{drivers::mongodb::TMongoClient, shared::{types::{ApiErrors, ResponsePasswordRecord}, jwt_service::Token, encryption::decrypt_password}};

use super::secrets_module::SecretRecordResponse;

// #[get("/secret/<user_id>?<record>&<page>&<service>&<key>&<limit>")]
// async fn search_secret_records(
//     db: &State<Box<dyn TMongoClient>>,
//     user_id: String,
//     record: Option<RecordTypes>,
//     page: Option<u64>,
//     service: Option<String>,
//     key: Option<String>,
//     limit: Option<i64>,
//     token: Token
// ) -> Result<Json<Vec<SecretRecordResponse>>, ApiErrors> {
//     // Validate user_id
//     let user_id = ObjectId::parse_str(user_id)
//     .map_err(|_| ApiErrors::BadRequest("Provided Id is not an object id".to_string()))?;
    
//     // Check to see if the provided user_id is the same as the token.id
//     if user_id != token.id {
//         return Err(ApiErrors::BadRequest("Not Authorized".to_string()))
//     }
//     // dbg!(&record);
//     let search_params = SearchParams::new(user_id, record, page, service, key, limit);

//     /* Call To component: component::search(db, search_params) */
//     // Inside search_params
//     let mut res = db.search_records(search_params).await?;

//     let mut record_vec = Vec::new();
//     dbg!(&record_vec);
//     while let Some(record) = res.next().await {
//         let mut record = record.map_err(|err| ApiErrors::ServerError(err.to_string()))?;
//         // Decrypt Password
//         record.password = decrypt_password(&record.password)?;

//         let id = record.id.ok_or(ApiErrors::ServerError("No Objectid".to_string()))?.to_string();

//         let user_id = record.user_id.ok_or(ApiErrors::ServerError("No Objectid".to_string()))?.to_string();

//         // add to vector
//         record_vec.push(ResponsePasswordRecord { 
//             id, 
//             service: record.service, 
//             password: record.password, 
//             email: record.email, 
//             username: record.username, 
//             user_id 
//         });
//     }
//     Ok(Json(record_vec))
// }

#[get("/password/<user_id>?<record>&<page>&<service>&<key>&<limit>")]
async fn search_password_records(
    db: &State<Box<dyn TMongoClient>>,
    user_id: String,
    record: Option<Vec<RecordTypes>>,
    page: Option<u64>,
    service: Option<String>,
    key: Option<String>,
    limit: Option<i64>,
    token: Token
) -> Result<Json<Vec<ResponsePasswordRecord>>, ApiErrors> {
    // Validate user_id
    let user_id = ObjectId::parse_str(user_id)
        .map_err(|_| ApiErrors::BadRequest("Provided Id is not an object id".to_string()))?;
    
    // Check to see if the provided user_id is the same as the token.id
    if user_id != token.id {
        return Err(ApiErrors::BadRequest("Not Authorized".to_string()))
    }
    
    let search_params = SearchParams::new(user_id,Some(RecordTypes::Password), page, service, key, limit);

    /* Call To component: component::search(db, search_params) */
    // Inside search_params
    let mut res = db.search_records(search_params).await?;

    let mut record_vec = Vec::new();
    dbg!(&record_vec);
    while let Some(record) = res.next().await {
        let mut record = record.map_err(|err| ApiErrors::ServerError(err.to_string()))?;
        // Decrypt Password
        record.password = decrypt_password(&record.password)?;

        let id = record.id.ok_or(ApiErrors::ServerError("No Objectid".to_string()))?.to_string();

        let user_id = record.user_id.ok_or(ApiErrors::ServerError("No Objectid".to_string()))?.to_string();

        // add to vector
        record_vec.push(ResponsePasswordRecord { 
            id, 
            service: record.service, 
            password: record.password, 
            email: record.email, 
            username: record.username, 
            user_id 
        });
    }
    Ok(Json(record_vec))
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        search_password_records
    ]
}