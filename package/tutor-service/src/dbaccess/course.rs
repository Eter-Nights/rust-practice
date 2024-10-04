use crate::errors::TutorError;
use crate::models::course::*;
use sqlx::postgres::PgPool;
use std::string::String;

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, TutorError> {
    let course = sqlx::query_as!(Course, "INSERT into course (
        tutor_id, course_name, course_description,
        course_duration, course_level, course_format,
        course_language, course_structure, course_price)
        values ($1,$2,$3,$4,$5,$6,$7,$8,$9)
        returning tutor_id, course_id, course_name,
        course_description, course_duration, course_level,
        course_format, course_language, course_structure, course_price, posted_time",
        new_course.tutor_id, new_course.course_name, new_course.course_description,
        new_course.course_duration, new_course.course_level, new_course.course_format,
        new_course.course_language, new_course.course_structure, new_course.course_price)
        .fetch_one(pool)
        .await?;

    Ok(course)
}

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, TutorError> {
    let courses = sqlx::query_as!(Course,
        "SELECT * FROM course where tutor_id = $1 order by course_id desc",
        tutor_id
    )
        .fetch_all(pool)
        .await?;

    Ok(courses)
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, TutorError> {
    let course = sqlx::query_as!(Course,
        "SELECT * FROM course where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
        .fetch_one(pool)
        .await;

    course.or(Err(TutorError::NotFound("Course id not found".into())))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, TutorError> {
    let current_course = sqlx::query_as!(
        Course,
        "SELECT * FROM course where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
        .fetch_one(pool)
        .await
        .map_err(|_err| TutorError::NotFound("Course id not found".into()))?;

    let name = update_course.course_name.or(Some(current_course.course_name)).unwrap_or_default();
    let description = update_course.course_description.or(current_course.course_description).unwrap_or_default();
    let format = update_course.course_format.or(current_course.course_format).unwrap_or_default();
    let structure = update_course.course_structure.or(current_course.course_structure).unwrap_or_default();
    let duration = update_course.course_duration.or(current_course.course_duration).unwrap_or_default();
    let price = update_course.course_price.or(current_course.course_price).unwrap_or_default();
    let language = update_course.course_language.or(current_course.course_language).unwrap_or_default();
    let level = update_course.course_level.or(current_course.course_level).unwrap_or_default();

    let course =
        sqlx::query_as!(
        Course,
        "UPDATE course set course_name = $1, course_description = $2, course_format = $3,
        course_structure = $4, course_duration = $5, course_price = $6, course_language = $7,
        course_level = $8 where tutor_id = $9 and course_id = $10 returning tutor_id, course_id,
        course_name, course_description, course_duration, course_level, course_format,
        course_language, course_structure, course_price, posted_time ", name, description, format,
        structure, duration, price, language, level, tutor_id, course_id
    )
            .fetch_one(pool)
            .await;

    course.or(Err(TutorError::NotFound("Course id not found".into())))
}

pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, TutorError> {
    let res = sqlx::query!(
        "DELETE FROM course where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id,
    )
        .execute(pool)
        .await?;

    match res.rows_affected() {
        0 => Err(TutorError::NotFound("Course id not found".into())),
        _ => Ok(format!("Deleted {} record", res.rows_affected()))
    }
}
