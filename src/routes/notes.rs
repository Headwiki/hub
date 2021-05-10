use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    note: String
}

pub async fn add_note(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}