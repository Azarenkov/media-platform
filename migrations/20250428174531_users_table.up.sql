-- Add up migration script here
create table users (
    id uuid primary key default gen_random_uuid (),
    email text unique not null,
    username text not null,
    password text not null
);
