use crate::connection::{establish_connection};
use crate::diesel::insert_into;
use crate::entities::user::User;
use diesel::{RunQueryDsl};
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::user;

pub fn create_user() {

    let conn = establish_connection();
    insert_into(user::table)
        .default_values()
        .execute(&conn);
}

pub fn find_user(id: Uuid) -> User {
    let conn = establish_connection();
    
    match user::table.find(id).get_result::<User>(&conn) {
        Ok(usr) => return usr,
        Err(e) => todo!()
    }
}