use actix_web::{web, App, HttpServer, HttpResponse};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}