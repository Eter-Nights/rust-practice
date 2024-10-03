use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = move || {
        App::new()
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index))
            )
    };
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
