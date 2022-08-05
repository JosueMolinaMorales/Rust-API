use std::env;

pub fn get_db_uri() -> String {
    match env::var("MONGODB_URI") {
        Ok(res) => res,
        Err(_) => {
            panic!("MONGODB_URI Env Not Set!")
        }
    }
}

pub fn get_db_name() -> String {
    env::var("DB_NAME").expect("DB_NAME Env not set!")
}