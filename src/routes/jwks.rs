use actix_web::{web, HttpResponse, Responder};
use log::{debug, trace};
use mongodb::{Database, bson::doc, Collection};
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::AppState;
use crate::utils::jwksmanager::{JWK, JWKDoc};


const JWKS: &str = "JWKS";

#[derive(Debug, Serialize, Deserialize)]
pub struct JWKS {
    #[serde(rename = "keys")]
    pub keys: Vec<JWK>,
}

impl JWKS{
    pub fn build(mongo_db: Database) -> Option<JWKS>{
        let jwks_coll: Collection = mongo_db.collection(JWKS);
        let filter = doc! {"current": true};
        match jwks_coll.find(filter, None) {
            Ok(cursor) => {
                None
            },
            Err(e) => None
        }
    }
}


pub async fn execute(
    app_state: web::Data<AppState>,
) -> impl Responder {
    let mongo_db = &app_state.mongo_db;
    match JWKS::build(mongo_db){
        Some(JWKS){

        },
        None {
            HttpResponse::NotFound()
        }
    }
    HttpResponse::Ok().json(user_name.precheck(mongo_db))
}
