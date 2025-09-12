use crate::entities::{self, users};
use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, get, post, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use log;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserInfoAnswer {
    username: String,
    email: String,
}

fn hash_password(password: &str) -> Result<(String, String), argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok((salt.to_string(), password_hash))
}

#[post("/register")]
async fn register(
    db: web::Data<DatabaseConnection>,
    json: web::Json<RegisterRequest>,
    request: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let (salt, password_hashed) = hash_password(json.password.as_str())
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password hasing problem"))?;
    let user = entities::users::ActiveModel {
        username: sea_orm::ActiveValue::Set(json.username.to_owned()),
        email: sea_orm::ActiveValue::Set(json.email.to_owned()),
        password_salt: sea_orm::ActiveValue::Set(salt.to_owned()),
        password_hashed: sea_orm::ActiveValue::Set(password_hashed.to_owned()),
        ..Default::default()
    };
    if let Ok(Some(user)) = entities::prelude::Users::find()
        .filter(users::Column::Username.contains(json.username.to_owned()))
        .one(db.get_ref())
        .await
    {
        log::debug!("User already found: {user:?}");
        return Err(actix_web::error::ErrorConflict("Username already taken"));
    }

    if let Ok(Some(user)) = entities::prelude::Users::find()
        .filter(users::Column::Email.contains(json.email.to_owned()))
        .one(db.get_ref())
        .await
    {
        log::debug!("Email already found: {user:?}");
        return Err(actix_web::error::ErrorConflict("Email already taken"));
    }

    let res = entities::prelude::Users::insert(user)
        .exec(db.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    log::info!("User {} insert ok", json.username);
    Identity::login(&request.extensions(), res.last_insert_id.to_string()).unwrap();
    Ok(HttpResponse::Ok())
}

#[post("/login")]
async fn login(
    db: web::Data<DatabaseConnection>,
    request: HttpRequest,
    json: web::Json<LoginRequest>,
) -> actix_web::Result<impl Responder> {
    let user = entities::prelude::Users::find()
        .filter(users::Column::Email.contains(json.email.to_owned()))
        .one(db.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .ok_or(actix_web::error::ErrorNotFound("Incorrect email"))?;

    let parsed_hash = PasswordHash::new(&user.password_hashed)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password decryption error"))?;
    Argon2::default()
        .verify_password(json.password.as_bytes(), &parsed_hash)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Incorrect password"))?;
    Identity::login(&request.extensions(), user.id.to_string()).unwrap();
    Ok(HttpResponse::Ok())
}

#[post("/logout")]
async fn logout(user: Identity) -> actix_web::Result<impl Responder> {
    user.logout();
    Ok(HttpResponse::Ok())
}

#[get("/info")]
async fn info(
    user: Identity,
    db: web::Data<DatabaseConnection>,
) -> actix_web::Result<impl Responder> {
    let user = entities::prelude::Users::find_by_id(user.id().unwrap().parse::<i32>().unwrap())
        .one(db.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .ok_or(actix_web::error::ErrorInternalServerError("user not found"))?;
    let ans = UserInfoAnswer {
        username: user.username,
        email: user.email,
    };
    Ok(HttpResponse::Ok().json(ans))
}

pub fn scoped_identity_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity")
            .service(login)
            .service(logout)
            .service(register)
            .service(info),
    );
}
