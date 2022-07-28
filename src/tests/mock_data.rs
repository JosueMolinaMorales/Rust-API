use bson::oid::ObjectId;

use crate::shared::types::{User, PasswordRecord};
pub struct MockData {
    pub email_exists: String,
    pub email_not_exists: String,
    pub object_id: ObjectId,
    pub dne_object_id: ObjectId,
    pub a_record: PasswordRecord,
    pub a_user: User
}

impl MockData {
    pub fn new() -> MockData {
        MockData { 
            email_exists: "a_email@mail.com".to_string(), 
            email_not_exists: "not_exists@mail.com".to_string(), 
            object_id: ObjectId::new(), 
            dne_object_id: ObjectId::new(),
            a_record: PasswordRecord { 
                id: Some(ObjectId::new()), 
                service: "Netflix".to_string(), 
                password: "password".to_string(), 
                email: Some("email@gma.com".to_string()), 
                username: Some("username".to_string()), 
                user_id: Some(ObjectId::new()) 
            }, 
            a_user: User { 
                id: Some(ObjectId::new()), 
                name: "name".to_string(), 
                email: "email".to_string(), 
                username: "username".to_string(), 
                password: "password".to_string() 
            }
        }
    }
}