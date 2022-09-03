-- This file should undo anything in `up.sql`
alter table "user"
    drop column username;

alter table "user"
    drop column avatar_url;