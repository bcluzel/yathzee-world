use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::dev::{ServiceRequest, ServiceResponse, fn_service};
use actix_web::{App, HttpResponse, HttpServer, cookie::Key};
use log;
mod api;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("yathee-world init");
    let config = config::load_config();
    log::info!("Starting http server");

    let session_secret_key = Key::from(config.key.as_slice());

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                session_secret_key.clone(),
            ))
            .configure(api::scoped_api)
            .service(
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
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
