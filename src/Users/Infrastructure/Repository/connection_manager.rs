use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub struct ConnectionManager {
    pub connection_write: PgConnection,
    pub connection_read: PgConnection,
}

impl ConnectionManager {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url_write = env::var("DATABASE_URL_WRITE").expect("DATABASE_URL_WRITE must be set");
        let database_url_read = env::var("DATABASE_URL_READ").expect("DATABASE_URL_READ must be set");
        
        ConnectionManager {
            connection_write: PgConnection::establish(&database_url_write).expect(&format!("Error connecting to {}", database_url_write)),
            connection_read: PgConnection::establish(&database_url_read).expect(&format!("Error connecting to {}", database_url_read)),
        }
    }
}