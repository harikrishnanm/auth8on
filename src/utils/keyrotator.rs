use clokwerk::{Scheduler, TimeUnits};
use log::{error, info, trace};
use mongodb::{bson::doc, sync::Collection};
use std::{thread, time::Duration};

use crate::utils::jwksmanager::{JWKDoc, JWK};
use crate::AppState;

use crate::constants::JWKS;

pub fn start(app_state: AppState) {
    info!("Starting key rotation scheduler...");
    let mut scheduler = Scheduler::new();
    scheduler.every(300.seconds()).run(move || {
        info!("Rotating keys");
        let mongo_db = app_state.clone().mongo_db;
        let jwks_coll_typed: Collection<JWKDoc> = mongo_db.collection_with_type(JWKS);
        let jwks_coll: Collection = mongo_db.collection(JWKS);
        let jwks_count: i64 = jwks_coll.count_documents(None, None).unwrap();
        let active_filter = doc! {"current": true};
        let inactive_filter = doc! {"current": false};
        match jwks_coll.find_one(active_filter, None) {
            Ok(opt) => {
                match opt {
                    Some(mut old_doc) => {
                        let jwk_doc = JWKDoc::build(JWK::build().unwrap());
                        match jwks_coll_typed.insert_one(jwk_doc, None) {
                            Ok(result) => {
                                info!("Successfully stored JWKDoc");
                                trace!("Result {:?}", result);
                            }
                            Err(e) => {
                                error!("Error storing JWKDoc {:?}", e);
                            }
                        };
                        //Now update the old doc and insert that too.
                        old_doc.insert("current", false);
                        trace!("Old doc {:?}", old_doc.get("_id").unwrap());
                        let filter = doc! {"_id": old_doc.get("_id").unwrap()};
                        let update = doc! {"$set" : {"current": false}};
                        if jwks_count > 1 {
                            let _p = jwks_coll.find_one_and_delete(inactive_filter, None);
                        }
                        let _r = jwks_coll.find_one_and_update(filter, update, None);
                    }
                    None => {
                        error!("Error rotating keys coz the old one could not be updated...")
                    }
                }
            }
            Err(e) => error! {"Error rotating keys coz the old one is hiding...{:?}", e},
        };
    });
    //let thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(1));
    }
}
