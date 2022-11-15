use std::{collections::BTreeMap, io::Cursor};

use bson::Document;
use mongodb::bson::oid::ObjectId;
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    serde::json::{serde_json, Json},
    Request, Response,
};
use serde::{Deserialize, Serialize, Serializer};
use validator::Validate;

#[derive(Debug)]
pub enum ApiErrors {
    ServerError(String),
    BadRequest(String),
    Forbidden(String),
    Unauthorized(String),
    NotFound(String),
}
impl<'r> Responder<'r, 'r> for ApiErrors {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        let string = serde_json::to_string(&self).map_err(|e| {
            error_!("JSON failed to serialize: {:?}", e);
            Status::InternalServerError
        })?;

        let mut res = Response::build();
        match self {
            ApiErrors::ServerError(_) => {
                res.status(Status::InternalServerError)
                    .sized_body(string.len(), Cursor::new(string));
            }
            ApiErrors::BadRequest(_) => {
                res.status(Status::BadRequest)
                    .sized_body(string.len(), Cursor::new(string));
            }
            ApiErrors::Forbidden(_) => {
                res.status(Status::Forbidden)
                    .sized_body(string.len(), Cursor::new(string));
            }
            ApiErrors::Unauthorized(_) => {
                res.status(Status::Unauthorized)
                    .sized_body(string.len(), Cursor::new(string));
            }
            ApiErrors::NotFound(_) => {
                res.status(Status::NotFound)
                    .sized_body(string.len(), Cursor::new(string));
            }
        };
        res.header(ContentType::JSON);
        res.ok()
    }
}

impl Serialize for ApiErrors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut error_obj: BTreeMap<&str, BTreeMap<&str, &String>> = BTreeMap::new();
        let mut error_msg: BTreeMap<&str, &String> = BTreeMap::new();
        let server_error_msg = "Internal Service Error".to_string();
        match self {
            ApiErrors::ServerError(msg) => {
                // Log Error Message
                println!("{}", msg);
                error_msg.insert("message", &server_error_msg)
            }
            ApiErrors::BadRequest(msg) => error_msg.insert("message", msg),
            ApiErrors::Forbidden(msg) => error_msg.insert("message", msg),
            ApiErrors::Unauthorized(msg) => error_msg.insert("message", msg),
            ApiErrors::NotFound(msg) => error_msg.insert("message", msg),
        };
        error_obj.insert("error", error_msg);

        serializer.collect_map(error_obj.iter())
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationForm {
    pub name: String,

    #[validate(email)]
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthResponse {
    pub user: AuthUser,
    pub token: String,
}

pub struct PartialUser {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthUser {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRecord  {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub record_type: RecordTypes,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, FromFormField, Serialize, Deserialize)]
pub enum RecordTypes {
    Password,
    Secret,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseRecord {
    pub record_type: RecordTypes,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreatedResponse {
    pub id: Json<Document>,
}
