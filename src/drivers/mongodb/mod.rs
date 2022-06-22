use std::env;

use mongodb::{ Client, options::ClientOptions };

pub struct MongoClient {
    client: Option<mongodb::Client>
}

impl MongoClient {
    /**
     * Create a new MongoClient Struct
     */
    pub fn new() -> MongoClient {
        MongoClient {
            client: None
        }
    }

    /**
     * Connect to the Database
     */
    pub async fn connect(&mut self) {
        let db_uri = match env::var("MONGODB_URI") {
            Ok(res) => res,
            Err(_) => {
                panic!("MONGODB_URI Env Not Set!")
            }
        };
        let client_options = ClientOptions::parse(db_uri).await.expect("There was an error parsing the DB_URI");
        
        let client = Client::with_options(client_options).expect("There was an error connecting to the database");
        
        println!("Connection to mongodb established!");
        
        self.client = Some(client);
    } 

    /**
     * Get the client
     */
    pub fn get_client(&self) -> &mongodb::Client {
        match &self.client {
            Some(val) => return val,
            None => panic!("Connect to client not established!")
        }
    }
}
