use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    note: String
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
async fn add_note(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/notes", web::post().to(add_note))
    })
        .listen(listener)?
        .run();

    Ok(server)
}