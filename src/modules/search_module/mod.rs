/*
    Searching Module will be used for searching through password records and secret records
    /<user_id>/search?record=&page=&service=&key=
    * Record will be either secret or password
    * Page will be the pagination number
    * Service will be the name of the service to search for
    * Key will be the key of the secret to get
*/
use rocket::{futures::stream::StreamExt, serde::json::Json};

pub mod component;


#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "_id")]
    id: String,
    user_id: String,
    record_type: RecordTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<String>,
}

impl SearchResponse {
    pub fn new(
        id: String,
        user_id: String,
        record_type: RecordTypes,
        service: Option<String>,
        username: Option<String>,
        password: Option<String>,
        email: Option<String>,
        key: Option<String>,
        secret: Option<String>,
    ) -> SearchResponse {
        SearchResponse {
            id,
            user_id,
            record_type,
            username,
            service,
            password,
            email,
            key,
            secret,
        }
    }
}

pub struct SearchParamsBuilder {
    pub user_id: ObjectId,
    pub password_record: Option<RecordTypes>,
    pub secret_record: Option<RecordTypes>,
    pub page: Option<u64>,
    pub query: Option<String>,
    pub limit: Option<i64>,
}

impl SearchParamsBuilder {
    pub fn new(user_id: ObjectId) -> SearchParamsBuilder {
        SearchParamsBuilder {
            user_id,
            password_record: None,
            secret_record: None,
            query: None,
            page: None,
            limit: None,
        }
    }

    pub fn add_record(mut self, record_type: RecordTypes) -> Self {
        match record_type {
            RecordTypes::Password => self.password_record = Some(RecordTypes::Password),
            RecordTypes::Secret => self.secret_record = Some(RecordTypes::Secret),
        };
        self
    }

    pub fn add_page(mut self, page: Option<u64>) -> Self {
        self.page = page;
        self
    }

    pub fn add_limit(mut self, limit: Option<i64>) -> Self {
        self.limit = limit;
        self
    }

    pub fn add_query(mut self, query: Option<String>) -> Self {
        self.query = query;
        self
    }

    pub fn build(self) -> SearchParams {
        SearchParams {
            user_id: self.user_id,
            password_record: self.password_record,
            secret_record: self.secret_record,
            page: self.page,
            query: self.query,
            limit: self.limit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub user_id: ObjectId,
    pub password_record: Option<RecordTypes>,
    pub secret_record: Option<RecordTypes>,
    pub page: Option<u64>,
    pub query: Option<String>,
    pub limit: Option<i64>,
}

use bson::oid::ObjectId;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{
    drivers::mongodb::mongo_trait::TMongoClient,
    shared::{
        jwt_service::Token,
        types::{ApiErrors, RecordTypes},
    },
};

#[get("/record/<user_id>?<page>&<limit>&<query>")]
async fn search_records(
    db: &State<Box<dyn TMongoClient>>,
    user_id: String,
    page: Option<u64>,
    query: Option<String>,
    limit: Option<i64>,
    token: Token,
) -> Result<Json<Vec<SearchResponse>>, ApiErrors>{
    // Validate user_id
    let user_id = ObjectId::parse_str(user_id)
        .map_err(|_| ApiErrors::BadRequest("Provided Id is not an object id".to_string()))?;

    // Check to see if the provided user_id is the same as the token.id
    if user_id != token.id {
        return Err(ApiErrors::BadRequest("Not Authorized".to_string()));
    }

    let search_params = SearchParamsBuilder::new(user_id)
        .add_limit(limit)
        .add_page(page)
        .add_query(query)
        .build();

    let records = component::search_records(db, search_params).await?;

    Ok(Json(records))
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![search_records]
}
