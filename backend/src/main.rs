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
    let host: String = std::env::var("HOST").unwrap_or("127.0.0.1".to_owned());
    let port: u16 = std::env::var("PORT")
        .map(|x| x.parse().expect("Incorrect port: not a number"))
        .unwrap_or(8080);
    log::info!("Starting http server listening to {host}:{port}");

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
    .bind((host, port))?
    .run()
    .await
}
