use actix_web::{web, HttpResponse, Responder};
use log::{debug, trace};
use mongodb::sync::Database;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::AppState;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Credentials {
    #[serde(alias = "username")]
    pub user_name: String,
    #[serde(alias = "password")]
    pub password: String,
}

impl Credentials {
    fn login(&self, mongo_db: &Database) {
        debug!("Attempting login");
        trace!("Credentials {:?}", self);
    }
}

pub async fn execute(
    credentials: web::Json<Credentials>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    trace!("Credentials object {:?}", credentials);
    let mongo_db = &app_state.mongo_db;
    HttpResponse::Ok().json(credentials.login(mongo_db))
}
