use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};

mod health_check;
use health_check::HealthCheckStatus;

async fn health_check(_req: HttpRequest) -> Result<impl Responder> {
    let status = HealthCheckStatus::new();

    Ok(web::Json(status))
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting HTTP server on port 8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
