use crate::dbaccess::tutor::*;
use crate::errors::TutorError;
use crate::models::tutor::{CreateTutor, UpdateTutor};
use crate::state::AppState;

use actix_web::{web, HttpResponse};

/*
curl -X POST localhost:8080/tutors/ -H "Content-Type: application/json" \
-d '{ "tutor_name":"Jessica", "tutor_pic_url":"test.com/pic1", "tutor_profile":"plus"}'
 */
pub async fn post_new_tutor(
    new_tutor: web::Json<CreateTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, TutorError> {
    post_new_tutor_db(&app_state.db, new_tutor.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, TutorError> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, TutorError> {
    let tutor_id = path.into_inner();
    get_tutor_details_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

/*
curl -X PUT localhost:8080/tutors/3 -H "Content-Type: application/json" \
-d '{ "tutor_name":"Jessica", "tutor_pic_url":"test.com/pic1111", "tutor_profile":"plus"}'
*/
pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
    update_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, TutorError> {
    let tutor_id = path.into_inner();
    update_tutor_details_db(&app_state.db, tutor_id, update_tutor.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

// curl -X DELETE localhost:8080/tutors/3
pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, TutorError> {
    let tutor_id = path.into_inner();
    delete_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenvy::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_tutor_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_tutor_msg = CreateTutor {
            tutor_name: "Third tutor".into(),
            tutor_pic_url: "test.com/pic4".into(),
            tutor_profile: "plus".into(),
        };
        let tutor_param = web::Json(new_tutor_msg);
        let resp = post_new_tutor(tutor_param, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_tutors_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let resp = get_all_tutors(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_tutor_detail_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<i32> = web::Path::from(3);
        let resp = get_tutor_details(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_tutor_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<i32> = web::Path::from(2);
        let resp = delete_tutor(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
