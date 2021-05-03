use log::{error, info, trace};
use mongodb::{bson::doc, error::Error, options::ClientOptions, Client, Database};
use std::result::Result;

use std::env;

pub async fn init() -> Result<Database, Error> {
    info!("Initializing DB");
    const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
    const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let mongo_url = match env::var("MONGO_URL") {
        Ok(mut url) => {
            if !url.contains("appName") {
                url.push_str("&appName=");
                url.push_str(CARGO_PKG_NAME);
            }
            url
        }
        Err(_e) => {
            error!("MONGO_URL env variable not set. Will default to mongodb://localhost:27017");
            String::from("mongodb://lodcalhost:27017/")
        }
    };

    let mongo_db_name = match env::var("MONGO_DB_NAME") {
        Ok(db_name) => db_name,
        Err(e) => panic!("MONGO_DB_NAME env variable not set {}", e),
    };

    match Client::with_uri_str(&mongo_url).await {
        Ok(client) => {
            match client
                .database("admin")
                .run_command(doc! {"ping": 1}, None)
                .await
            {
                Ok(_) => {
                    let db = client.database(&mongo_db_name);

                    info!("DB Initialized");
                    Ok(db)
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

async fn check_and_create_collections(mongo_db: &Database) {
    let collections = mongo_db.list_collections(None, None).await;
    trace!("{:?}", collections);
}
