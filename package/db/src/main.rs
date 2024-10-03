use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use std::env;
use std::io;

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenvy::from_path("package/db/.env").unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let course_rows = sqlx::query!(
        r#"select course_id, tutor_id, course_name, posted_time from course where course_id = $1"#,
        1
    )
        .fetch_all(&db_pool)
        .await
        .unwrap();

    let mut course_list = vec![];
    for row in course_rows {
        course_list.push(Course {
            course_id: row.course_id,
            tutor_id: row.tutor_id,
            course_name: row.course_name,
            posted_time: Some(chrono::NaiveDateTime::from(row.posted_time.unwrap())),
        })
    }

    println!("Course = {:?}", course_list);
    Ok(())
}
