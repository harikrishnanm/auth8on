use actix_web::web;

mod jwks;
mod login;
mod precheck;

pub fn register(app: &mut web::ServiceConfig) {
    app.service(web::resource("/precheck").route(web::post().to(precheck::execute)));
    app.service(web::resource("/.well_known/jwks").route(web::get().to(jwks::execute)));
}
