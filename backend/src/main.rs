use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
#[cfg(feature = "staticserve")]
use actix_web::HttpResponse;
#[cfg(feature = "staticserve")]
use actix_web::dev::{ServiceRequest, ServiceResponse, fn_service};
use actix_web::middleware;
use actix_web::{App, HttpServer, cookie::Key, web};

use log;
use sea_orm::Database;
use sea_orm_migration::prelude::*;
use std::fs;
mod api;
mod config;
mod migrator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("yathee-world init");
    let workdir_path = "./workdir".to_owned();
    if !fs::exists(&workdir_path).unwrap() {
        log::info!("Workdir not found, creating.");
        fs::create_dir(&workdir_path).unwrap();
    }
    let database_url = format!("sqlite:{workdir_path}/app.db?mode=rwc");
    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");
    log::info!("Database connected");
    let config = config::load_config(&workdir_path);
    log::info!("Loaded config");
    migrator::Migrator::refresh(&db).await.unwrap();
    log::info!("Database configured");
    let host: String = std::env::var("HOST").unwrap_or("127.0.0.1".to_owned());
    let port: u16 = std::env::var("PORT")
        .map(|x| x.parse().expect("Incorrect port: not a number"))
        .unwrap_or(8080);
    log::info!("Starting http server listening to {host}:{port}");

    let session_secret_key = Key::from(config.key.as_slice());

    HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                session_secret_key.clone(),
            ))
            .wrap(middleware::Logger::default())
            .configure(api::scoped_api);

        #[cfg(feature = "staticserve")]
        let app = app.service(
            actix_files::Files::new("/", "./static-frontend")
                .prefer_utf8(true)
                .index_file("index.html")
                .default_handler(fn_service(|req: ServiceRequest| async {
                    let (req, _) = req.into_parts();
                    Ok(ServiceResponse::new(
                        req,
                        HttpResponse::NotFound().body("Build failed, contact website owner"),
                    ))
                })),
        );

        app
    })
    .bind((host, port))?
    .run()
    .await
}
