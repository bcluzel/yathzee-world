use actix_web::dev::{ServiceRequest, ServiceResponse, fn_service};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").service(hello))
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
