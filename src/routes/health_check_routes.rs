use actix_web::{Responder, HttpResponse, get};
use crate::structs::responses::GenericResponse;

#[get("/api/healthcheck")]
pub async fn health_check_handler() -> impl Responder {
    const MESSAGE: &str = "Server is healthy";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}