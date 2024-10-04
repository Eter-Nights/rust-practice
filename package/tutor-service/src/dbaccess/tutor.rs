use crate::errors::TutorError;
use crate::models::tutor::*;
use sqlx::postgres::PgPool;

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: CreateTutor) -> Result<Tutor, TutorError> {
    let tutor = sqlx::query_as!(Tutor, "insert into tutor (tutor_name, tutor_pic_url, tutor_profile)
        values ($1,$2,$3) returning tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor.tutor_name, new_tutor.tutor_pic_url, new_tutor.tutor_profile)
        .fetch_one(pool)
        .await?;

    Ok(tutor)
}

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, TutorError> {
    let tutors = sqlx::query_as!(Tutor, "SELECT * FROM tutor")
        .fetch_all(pool)
        .await?;

    Ok(tutors)
}

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, TutorError> {
    let tutor = sqlx::query_as!(Tutor,
        "SELECT * FROM tutor where tutor_id = $1 order by tutor_id desc",
        tutor_id
    )
        .fetch_one(pool)
        .await;

    tutor.or(Err(TutorError::NotFound("Tutor id not found".into())))
}

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    change_tutor: UpdateTutor,
) -> Result<Tutor, TutorError> {
    let tutor = sqlx::query_as!(Tutor, "SELECT * FROM tutor where tutor_id = $1",
        tutor_id
    )
        .fetch_one(pool)
        .await?;

    let new_tutor = Tutor {
        tutor_id: tutor.tutor_id,
        tutor_name: change_tutor.tutor_name.or(Some(tutor.tutor_name)).unwrap(),
        tutor_pic_url: change_tutor.tutor_pic_url.or(Some(tutor.tutor_pic_url)).unwrap(),
        tutor_profile: change_tutor.tutor_profile.or(Some(tutor.tutor_profile)).unwrap(),
    };

    let new_tutor = sqlx::query_as!(Tutor,
        "UPDATE tutor SET tutor_name = $1, tutor_pic_url=$2, tutor_profile=$3 where tutor_id = $4
        returning tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor.tutor_name, new_tutor.tutor_pic_url, new_tutor.tutor_profile, new_tutor.tutor_id
    )
        .fetch_one(pool)
        .await;

    new_tutor.or(Err(TutorError::NotFound("Tutor id not found".into())))
}

pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<String, TutorError> {
    let res = sqlx::query!("DELETE FROM tutor where tutor_id = $1",tutor_id)
        .execute(pool)
        .await?;

    match res.rows_affected() {
        0 => Err(TutorError::NotFound("Tutor id not found".into())),
        _ => Ok(format!("Deleted {} record", res.rows_affected()))
    }
}
