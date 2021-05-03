use actix_http::ResponseBuilder;
use actix_web::{
    error,
    http::StatusCode,
    middleware::{Compress, Logger},
    App, HttpServer,
};
use actix_web_validator::{error::Error::Validate, JsonConfig};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use mongodb::Database;
use std::thread;

mod config;
mod db;
mod routes;
mod scheduler;
mod utils;

#[derive(Clone)]
pub struct AppState {
    mongo_db: Database,
}

fn handle_err(err: actix_web_validator::error::Error) -> actix_http::error::Error {
    match err {
        Validate(validation_err) => {
            let rs = ResponseBuilder::new(StatusCode::BAD_REQUEST).json(validation_err.clone());
            error::InternalError::from_response(validation_err, rs).into()
        }
        err => {
            error!("Error processing json {}", &err);
            error::InternalError::new(err, StatusCode::BAD_REQUEST).into()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    info!("########   Starting Authentication Service   #########");

    let addr = config::get_server_address();
    let workers = config::get_worker_count();

    info!("Server Address: {}", &addr);
    info!("Worker threads: {}", &workers);

    let mongo_db = match db::init().await {
        Ok(mongo_db) => mongo_db,
        Err(e) => {
            error!("Could not initialize DB, Cannot continue");
            panic!("{:?}", e)
        }
    };

    let app_state: AppState = AppState { mongo_db: mongo_db };

    utils::jwksmanager::init_jwk(&app_state).await;
    // thread::spawn(|| {
    //     scheduler::start(&app_state);
    // });

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .app_data(JsonConfig::default().error_handler(|err, _req| handle_err(err)))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(routes::register)
    })
    .workers(workers)
    .bind(addr)?
    .run()
    .await
}
