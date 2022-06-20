use std::env;

use mongodb::{ Client, options::ClientOptions };

pub struct Mongo_client {
    client: Option<mongodb::Client>
}

impl Mongo_client {
    pub fn new() -> Mongo_client {
        Mongo_client {
            client: None
        }
    }

    pub async fn connect(&mut self) -> &mut Mongo_client {
        // Check to see if a connection to the client has been established
        match &self.client {
            Some(val) => return self,
            None => {}
        }
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

        self
    } 

    pub fn get_client(&self) -> &mongodb::Client {
        match &self.client {
            Some(val) => return val,
            None => panic!("Connect to client not established!")
        }
    }
}
