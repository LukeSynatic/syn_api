use actix_web::{Responder, HttpResponse, get, web::{scope, ServiceConfig}};
use crate::structs::responses::GenericResponse;

pub fn configure_stats_service(cfg: &mut ServiceConfig) {
    cfg.service(
scope("/stats")
        .service(health_check_handler)
    );
}

#[get("/healthcheck")]
async fn health_check_handler() -> impl Responder {
    const MESSAGE: &str = "Server is healthy";

    let response_json = &GenericResponse {
        status_code: 200,
        message: MESSAGE.to_string(),
    };

    HttpResponse::Ok().json(response_json)
}