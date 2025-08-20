use actix_web::dev::{ServiceRequest, ServiceResponse, fn_service};
use actix_web::{App, HttpResponse, HttpServer};
use log;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("yathee-world init");
    HttpServer::new(|| {
        App::new().configure(api::scoped_api).service(
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
