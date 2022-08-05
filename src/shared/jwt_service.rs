use jwt::{SignWithKey, VerifyWithKey};
use mongodb::bson::oid::ObjectId;
use rocket::{Request, request::{FromRequest, self, Outcome}, http::Status};
use std::collections::BTreeMap;
use hmac::{ Hmac, Mac };
use sha2::Sha256;

use super::types::ApiErrors;

pub fn get_token_from_header(auth_header: String) -> Option<String> {
    println!("{}", auth_header);
    let mut auth_split = auth_header.split(" ");
    let bearer = auth_split.next();
    let token = auth_split.next();
    if bearer == Some("Bearer") {
        if let Some(a_token) = token {
            return Some(a_token.to_string());
        }
    }
    None
}

/**
 * Sign a token
 */
pub fn sign_token(object_id: &String) -> Result<String, ApiErrors> {
    let key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-secret") {
        Ok(res) => res,
        Err(_) => return Err(ApiErrors::ServerError("Issuing creating key".to_string()))
    };
    let mut claim = BTreeMap::new();
    claim.insert("id".to_string(), object_id);

    let signed_token = claim.sign_with_key(&key);

    match signed_token {
        Ok(token) => Ok(token),
        Err(_) => Err(ApiErrors::ServerError("Problem signing token".to_string()))
    }
}

pub fn verify_token(token: String) -> Result<ObjectId, ApiErrors> {
    let key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-secret") {
        Ok(res) => res,
        Err(_) => return Err(ApiErrors::ServerError("Issue creating key".to_string()))
    };

    let claims: BTreeMap<String, String> = match token.verify_with_key(&key) {
        Ok(token) => token,
        Err(_) => { return Err(ApiErrors::Unauthorized("Token is invalid".to_string()))},
    };

    if let Ok(id) = ObjectId::parse_str(claims["id"].as_str()) {
        Ok(id)
    } else {
        Err(ApiErrors::ServerError("Id is not in token".to_string()))
    }
}

#[derive(Debug)]
pub struct Token {
    pub id: ObjectId
}

#[async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiErrors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error>{
        // Get Auth Header if exists
        let auth_header = match request.headers().get_one("Authorization") {
            Some(header) => header.to_string(),
            None => return Outcome::Failure((Status::Unauthorized, ApiErrors::Unauthorized("Authorization Header Required".to_string())))
        };

        // Get the token from header
        let token = match get_token_from_header(auth_header) {
            Some(a_token) => a_token,
            None => return Outcome::Failure((Status::Unauthorized, ApiErrors::Unauthorized("Authorization Header Malformed".to_string())))
        };

        // Verify the JWT
        let id = match  verify_token(token) {
            Ok(id) => id,
            Err(_) => return Outcome::Failure((Status::Unauthorized, ApiErrors::Unauthorized("Failed to verify token".to_string())))
        };

        // Return the token
        Outcome::Success(Token {
            id
        })
    }
}
