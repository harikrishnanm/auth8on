use actix_web::{web, HttpResponse, Responder};
use log::{debug, info, trace};
use mongodb::{
    bson::doc,
    sync::{Collection, Cursor, Database},
};
use serde::{Deserialize, Serialize};

use crate::utils::jwksmanager::{JWKDoc, JWK};
use crate::AppState;

const JWKS: &str = "JWKS";

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicJWK {
    #[serde(rename = "n")]
    pub n: String,

    #[serde(rename = "kid")]
    pub kid: String,

    #[serde(rename = "kty")]
    pub kty: String,

    #[serde(rename = "alg")]
    pub alg: String,

    #[serde(rename = "e")]
    pub e: String,

    #[serde(rename = "use")]
    pub jwk_use: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWKS {
    #[serde(rename = "keys")]
    pub keys: Vec<PublicJWK>,
}

impl JWKS {
    pub async fn build(mongo_db: &Database) -> JWKS {
        let jwks_coll: Collection<JWKDoc> = mongo_db.collection_with_type(JWKS);
        let mut cursor: Cursor<JWKDoc> = jwks_coll.find(None, None).unwrap();
        let mut jwk_list: Vec<PublicJWK> = Vec::new();
        debug!("Got cursor to jwks");
        while let Some(doc) = cursor.next() {
            let jwk_doc: JWKDoc = doc.unwrap();
            let jwk: JWK = jwk_doc.jwk;
            let public_jwk: PublicJWK = PublicJWK {
                alg: jwk.alg,
                e: jwk.e,
                kid: jwk.kid,
                kty: jwk.kty,
                jwk_use: jwk.jwk_use,
                n: jwk.n,
            };
            jwk_list.push(public_jwk);
        }
        let jwks = JWKS { keys: jwk_list };
        trace!("JWKS {:?}", jwks);
        jwks
    }
}

pub async fn execute(app_state: web::Data<AppState>) -> impl Responder {
    info!("Getting jwks from db");
    let mongo_db = &app_state.mongo_db;
    let res = JWKS::build(mongo_db).await;
    HttpResponse::Ok().json(res)
}
