mod state;
mod routes;
mod errors;
mod models;
mod dbaccess;
mod handlers;

use crate::errors::TutorError;
use crate::routes::*;
use crate::state::AppState;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::sync::Mutex;
use std::{env, io};


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenvy::from_path("package/tutor-service/.env").unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let share_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(share_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                TutorError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    };

    let hostname_port = env::var("HOST_PORT").expect("DATABASE_URL must be set");
    HttpServer::new(app).bind(hostname_port)?.run().await
}
