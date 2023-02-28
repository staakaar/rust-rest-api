-- Your SQL goes here
create table articles (
    id integer auto_increment primary key,
    title varchar(255) not null,
    description varchar(1000),
    genre varchar(255),
    state integer,
    created_at timestamp default CURRENT_TIMESTAMP,
    updated_at timestamp default CURRENT_TIMESTAMP
);