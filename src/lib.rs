use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};
use actix_web::dev::Server;

mod health_check;
use health_check::HealthCheckStatus;

pub fn run() -> Result<Server, std::io::Error> {
    println!("Starting HTTP server on port 8080");

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}

pub async fn health_check(_req: HttpRequest) -> Result<web::Json<HealthCheckStatus>> {
    let status = HealthCheckStatus::new();

    Ok(web::Json(status))
}

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

