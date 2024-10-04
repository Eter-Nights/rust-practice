/* Drop tables if they already exist*/

drop table if exists course cascade;
drop table if exists tutor;

/* Create tables. */
/* Note: Don't put a comma after last field */

create table tutor
(
    tutor_id      serial primary key,
    tutor_name    varchar(200)  not null,
    tutor_pic_url varchar(200)  not null,
    tutor_profile varchar(2000) not null
);

create table course
(
    course_id          serial primary key,
    tutor_id           INT          not null,
    course_name        varchar(140) not null,
    course_description varchar(2000),
    course_format      varchar(30),
    course_structure   varchar(200),
    course_duration    varchar(30),
    course_price       INT,
    course_language    varchar(30),
    course_level       varchar(30),
    posted_time        TIMESTAMP default now(),
    CONSTRAINT fk_tutor
        FOREIGN KEY (tutor_id)
            REFERENCES tutor (tutor_id)
            ON DELETE cascade
);

SELECT setval('course_course_id_seq', 1);
SELECT setval('tutor_tutor_id_seq', 1);

/* Load seed data for testing */
insert into tutor(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
values (1, 'Merlene', 'test.com/pic1', 'plus');

insert into tutor(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
values (2, 'Frank', 'test.com/pic2', 'plus');

insert into tutor(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
values (3, 'Bob', 'test.com/pic3', 'plus');



insert into course
    (course_id, tutor_id, course_name, course_level, posted_time)
values (1, 1, 'First course from author 1', 'Beginner', '2021-04-12 05:40:00');
insert into course
    (course_id, tutor_id, course_name, course_format, posted_time)
values (2, 2, 'Second course from author 2', 'ebook', '2021-04-12 05:45:00');

insert into course
    (course_id, tutor_id, course_name, course_format, posted_time)
values (3, 1, 'Second course from author 1', 'ebook', '2021-04-12 05:45:00');

insert into course
    (course_id, tutor_id, course_name, course_format, posted_time)
values (4, 1, 'Third course from author 1', 'ebook', '2021-04-12 05:45:00');

insert into course
    (course_id, tutor_id, course_name, course_format, posted_time)
values (5, 3, 'First course from author 3', 'ebook', '2021-04-12 05:45:00');

SELECT setval('course_course_id_seq', (SELECT MAX(course_id) FROM course) + 1);
SELECT setval('tutor_tutor_id_seq', (SELECT MAX(tutor_id) FROM tutor) + 1);
