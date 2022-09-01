use crate::connection::{establish_connection};
use crate::diesel::insert_into;
use crate::entities::user::{User, NewUser};
use diesel::{RunQueryDsl, sql_query};
use diesel::prelude::*;
use crate::schema::user::{self, id};

pub fn create_user(newUser: NewUser) {
    let conn = establish_connection();

    insert_into(user::table)
    .values(newUser)
    .execute(&conn);
}

pub fn find_user(user_id: uuid::Uuid) -> Result<User, diesel::result::Error> {
    let conn = establish_connection();

    user::table.filter(id.eq_all(user_id)).first::<User>(&conn)
}