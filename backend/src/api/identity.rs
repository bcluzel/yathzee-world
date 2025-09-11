use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, post, web};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct RegisterRequest {
    id: String,
    name: String,
    password: String,
}

#[post("/register")]
async fn register(db: web::Data<DatabaseConnection>, request: HttpRequest) -> impl Responder {
    Identity::login(&request.extensions(), "User1".into()).unwrap();
    HttpResponse::Ok()
}

#[post("/login")]
async fn login(db: web::Data<DatabaseConnection>, request: HttpRequest) -> impl Responder {
    // Some kind of authentication should happen here
    // e.g. password-based, biometric, etc.
    // [...]

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

pub fn scoped_identity_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity")
            .service(login)
            .service(logout)
            .service(register),
    );
}
