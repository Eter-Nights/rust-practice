use crate::dbaccess::course::*;
use crate::errors::TutorError;
use crate::models::course::{CreateCourse, UpdateCourse};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

/*
curl -X POST localhost:8080/courses/ -H "Content-Type: application/json" \
 -d '{"tutor_id":2, "course_name":"Course 1"}'
*/
pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<CreateCourse>,
) -> Result<HttpResponse, TutorError> {
    post_new_course_db(&app_state.db, new_course.into_inner())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, TutorError> {
    let tutor_id = path.into_inner();
    get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, TutorError> {
    let (tutor_id, course_id) = path.into_inner();
    get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}


/*
curl -X PUT localhost:8080/courses/2/2 -H "Content-Type: application/json"  \
-d '{"course_name":"Valid course 3", "course_duration":"Its a long, long course"}'
*/
pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, TutorError> {
    let (tutor_id, course_id) = path.into_inner();
    update_course_details_db(&app_state.db, tutor_id, course_id, update_course.into_inner())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

// curl -X DELETE localhost:8080/courses/2/2
pub async fn delete_course(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, TutorError> {
    let (tutor_id, course_id) = path.into_inner();
    delete_course_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenvy::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    // Post a new course successfully
    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_course_msg = CreateCourse {
            tutor_id: 1,
            course_name: "Third course".into(),
            course_description: Some("This is a test course".into()),
            course_format: None,
            course_level: Some("Beginner".into()),
            course_price: None,
            course_duration: None,
            course_language: Some("English".into()),
            course_structure: None,
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(app_state, course_param).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // Get list of all courses for a tutor
    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // Get course details for valid course id.
    #[actix_rt::test]
    async fn get_course_detail_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_course_details(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // Get course details with invalid course id.
    #[actix_rt::test]
    async fn get_course_detail_failure_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 21));
        let resp = get_course_details(app_state, parameters).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    // Update course successfully
    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let update_course_msg = UpdateCourse {
            course_name: Some("Course name changed".into()),
            course_description: Some("This is yet another test course".into()),
            course_format: None,
            course_level: Some("Intermediate".into()),
            course_price: None,
            course_duration: None,
            course_language: Some("German".into()),
            course_structure: None,
        };
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let update_param = web::Json(update_course_msg);
        let resp = update_course_details(app_state, update_param, parameters)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // Delete course successfully
    #[actix_rt::test]
    async fn delete_test_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((3, 5));
        let resp = delete_course(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // Delete test failure
    #[actix_rt::test]
    async fn delete_test_failure() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 21));
        let resp = delete_course(app_state, parameters).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
