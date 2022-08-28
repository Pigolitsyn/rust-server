-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table "user"
(
    id    uuid default uuid_generate_v4() not null
        constraint id
            primary key,
    email varchar(100),
    hash  varchar(32)                     not null
);

create table "post"
(
    author_id uuid not null
        constraint author_id
            references "user" (id),
    id uuid default uuid_generate_v4() not null
    constraint key_name
        primary key,
    title varchar(255) not null,
    "text" text not null

);