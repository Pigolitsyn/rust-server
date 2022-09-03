-- This file should undo anything in `up.sql`

alter table "user"
    alter column avatar_url type text using avatar_url::text;