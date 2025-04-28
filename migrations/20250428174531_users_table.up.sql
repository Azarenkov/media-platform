-- Add up migration script here
create table users (
    id SERIAL PRIMARY KEY,
    email varchar not null,
    username varchar not null,
    password varchar not null
);

create unique index user_email_idx on users (email);
