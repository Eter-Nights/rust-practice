use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(general_routes);

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
