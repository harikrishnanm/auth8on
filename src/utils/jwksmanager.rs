use chrono::naive::NaiveDateTime;
use chrono::prelude::Utc;
use data_encoding::BASE64URL_NOPAD;
use log::{error, info, trace};
use diesel::{Queryable, Insertable};
use openssl::rsa::Rsa;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

use crate::AppState;
use crate::schema::jwks;


const JWKS: &str = "JWKS";
const BITS: u32 = 2048;



#[derive(Debug, Queryable, Insertable)]
#[table_name="jwks"]
pub struct JWK {
    pub created: NaiveDateTime,
    pub current: bool,
    pub p: String,
    pub q: String,
    pub d: String,
    pub qi: String,
    pub dp: String,
    pub dq: String,
    pub n: String,
    pub kty: String,
    pub e: String,
    pub jwk_use: String,
    pub alg: String,
    pub kid: String,
}

impl JWKDoc {
    pub fn build(jwk: JWK) -> JWKDoc {
        info!("Generating JWKDoc");
        let now = Utc::now().naive_utc();
        let jwk_doc = JWKDoc {
            jwk: jwk,
            created_date: now,
            current: true,
        };
        jwk_doc
    }
}

impl JWK {
    pub fn build() -> Result<JWK, Box<dyn Error>> {
        info!("Generating JWK");
        match Rsa::generate(BITS) {
            Ok(rsa) => {
                let p = rsa.p();
                let p_bytes = p.unwrap().to_vec();
                let p_base64 = BASE64URL_NOPAD.encode(p_bytes.as_slice());

                let q = rsa.q();
                let q_bytes = q.unwrap().to_vec();
                let q_base64 = BASE64URL_NOPAD.encode(q_bytes.as_slice());

                let d = rsa.d();
                let d_bytes = d.to_vec();
                let d_base64 = BASE64URL_NOPAD.encode(d_bytes.as_slice());

                let qi = rsa.iqmp();
                let qi_bytes = qi.unwrap().to_vec();
                let qi_base64 = BASE64URL_NOPAD.encode(qi_bytes.as_slice());

                let dp = rsa.dmp1();
                let dp_bytes = dp.unwrap().to_vec();
                let dp_base64 = BASE64URL_NOPAD.encode(dp_bytes.as_slice());

                let dq = rsa.dmq1();
                let dq_bytes = dq.unwrap().to_vec();
                let dq_base64 = BASE64URL_NOPAD.encode(dq_bytes.as_slice());

                let n = rsa.n();
                let n_bytes = n.to_vec();
                let n_base64 = BASE64URL_NOPAD.encode(n_bytes.as_slice());

                let e = rsa.e();
                let e_bytes = e.to_vec();
                let e_base64 = BASE64URL_NOPAD.encode(e_bytes.as_slice());

                let size: u32 = rsa.size();
                let mut alg = String::from("RS");
                alg.push_str(&*size.to_string());

                let jwk_use = String::from("sig");
                let kty = String::from("RSA");

                let kid = Uuid::new_v4().to_string();

                trace!("n {}", n_base64);
                trace!("e {}", e_base64);
                trace!("kid {}", kid);
                trace! {"size {}", size};

                let jwk = JWK {
                    p: p_base64,
                    q: q_base64,
                    d: d_base64,
                    qi: qi_base64,
                    dp: dp_base64,
                    dq: dq_base64,
                    n: n_base64,
                    e: e_base64,
                    jwk_use: jwk_use,
                    kid: kid,
                    kty: kty,
                    alg: alg,
                };
                trace!("JWK {:?}", jwk);
                Ok(jwk)
            }
            Err(e) => {
                error!("Could not generare key {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}

pub async fn init_jwk(app_state: &AppState) {
    info!("Checking JWKS");
    let mongo_db = app_state.clone().mongo_db;
    let jwks_coll: Collection<JWKDoc> = mongo_db.collection_with_type(JWKS);
    let jwks_count: i64 = jwks_coll.count_documents(None, None).unwrap();
    trace!("JWKS collection has {} docs", jwks_count);

    match JWK::build() {
        Ok(jwk) => {
            let jwk_doc = JWKDoc::build(jwk);
            match jwks_coll.insert_one(jwk_doc, None) {
                Ok(result) => {
                    info!("Successfully stored JWKDoc");
                    trace!("Result {:?}", result);
                }
                Err(e) => {
                    error!("Error storing JWKDoc {:?}", e);
                }
            }
        }
        Err(_e) => {}
    };
}
