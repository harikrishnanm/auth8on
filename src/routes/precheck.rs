use actix_web::{web, HttpResponse, Responder};
use log::{debug, trace};
use mongodb::sync::Database;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::AppState;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct PreCheck {
    #[serde(alias = "username")]
    pub user_name: String,
}

impl PreCheck {
    fn precheck(&self, mongo_db: &Database) -> Option<PreCheckResponse> {
        debug!("Precheck for {:?}", self);
        let user_name_value: String = self.user_name.to_string();

        let coll = mongo_db.collection("USER");
        //coll.
        Some(PreCheckResponse {
            user_name: user_name_value,
            password_cleared: true,
            user_enabled: true,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct PreCheckResponse {
    user_name: String,
    password_cleared: bool,
    user_enabled: bool,
}

pub async fn execute(
    user_name: web::Json<PreCheck>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    trace!("Username object {:?}", user_name);
    let mongo_db = &app_state.mongo_db;
    HttpResponse::Ok().json(user_name.precheck(mongo_db))
}
