/* Drop table if it already exists*/
drop table if exists course;
/* Create table. */
/* Note: Don't put a comma after last field */
create table course
(
    course_id   serial primary key,
    tutor_id    INT          not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

/* Load seed data for testing */
insert into course
    (course_id, tutor_id, course_name, posted_time)
values (1, 1, 'First course', '2020-12-17 05:40:00');
insert into course
    (course_id, tutor_id, course_name, posted_time)
values (2, 1, 'Second course', '2020-12-18 05:45:00');