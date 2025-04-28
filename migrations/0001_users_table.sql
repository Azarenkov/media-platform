create table users (
    email varchar not null,
    username varchar not null,
    password varchar not null
);

create unique index user_email_idx on users (email);
