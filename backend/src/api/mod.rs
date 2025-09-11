use actix_web::{HttpResponse, Responder, get, web};
mod identity;

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("running")
}

pub fn scoped_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(status)
            .configure(identity::scoped_identity_api),
    );
}
