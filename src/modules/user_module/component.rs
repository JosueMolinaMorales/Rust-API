use bson::oid::ObjectId;
use pwhash::bcrypt;
use rocket::State;

use crate::{drivers::mongodb::mongo_trait::TMongoClient, shared::types::{AuthUser, ApiErrors, UpdateUser}};


pub async fn get_user(
    db: &State<Box<dyn TMongoClient>>,
    user_id: ObjectId
) -> Result<AuthUser, ApiErrors> {
    let user = db.get_user_by_id(user_id).await?;

    Ok(user)
}

pub async fn update_user(
    db: &State<Box<dyn TMongoClient>>,
    user_id: ObjectId,
    mut updated_user: UpdateUser
) -> Result<(), ApiErrors> {
    if updated_user.email.is_none() && updated_user.new_password.is_none() {
        return Err(ApiErrors::BadRequest("Email or Password is required".to_string()))
    }

    // Check Passwords
    let user = db.get_user_by_id(user_id).await?;
    let user_password = db.get_user(&user.email).await?.password;

    if !bcrypt::verify(updated_user.password.clone(), &user_password) {
        // Passwords do not match
        return Err(ApiErrors::BadRequest("Password is incorrect".to_string()))
    }

    // Hash password
    if let Some(password) = updated_user.new_password {
        updated_user.new_password = Some(bcrypt::hash(password).map_err(|err| ApiErrors::ServerError(err.to_string()))?)
    }

    db.update_user_fields(user_id, updated_user).await?;
    Ok(())
}