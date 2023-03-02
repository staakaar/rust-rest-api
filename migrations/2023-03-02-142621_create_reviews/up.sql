-- Your SQL goes here
create table reviews (
    id integer auto_increment primary key,
    article_id foreign_key,
    detail varchar(255) not null,
    created_at timestamp default CURRENT_TIMESTAMP,
    updated_at timestamp default CURRENT_TIMESTAMP
);