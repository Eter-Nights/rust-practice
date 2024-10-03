mod state;
mod routes;
mod handles;

use crate::routes::*;
use crate::state::AppState;
use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    let share_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(Vec::new()),
    });

    let app = move || {
        App::new()
            .app_data(share_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
