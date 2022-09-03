-- Your SQL goes here

alter table "user"
    alter column avatar_url type varchar(200) using avatar_url::varchar(200);
