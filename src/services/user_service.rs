use crate::connection::{establish_connection};
use crate::diesel::insert_into;
use crate::entities::user::{NewUser, UserDto};
use diesel::{RunQueryDsl};
use diesel::prelude::*;
use crate::schema::user::{self, id, hash};

pub fn create_user(new_user: NewUser) -> Result<UserDto, diesel::result::Error> {
    let conn = establish_connection();
    insert_into(user::table)
    .values(new_user)
    .get_result::<UserDto>(&conn)
}

pub fn find_user(user_id: uuid::Uuid) -> Result<UserDto, diesel::result::Error> {
    let conn = establish_connection();
    
    user::table.filter(id.eq_all(user_id)).first::<UserDto>(&conn)
}

pub fn check_user_hash(user: UserDto) {
    let conn = establish_connection();
    
    let password_hash = user::table.filter(id.eq_all(user.id)).select(hash).first::<String>(&conn);
}