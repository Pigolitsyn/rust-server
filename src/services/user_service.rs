use crate::connection::{establish_connection};
use crate::diesel::insert_into;
use crate::entities::user::{NewUser, UserDto};
use diesel::{RunQueryDsl};
use diesel::prelude::*;
use crate::schema::user::{self, id, email};
use sha256::digest;

pub fn create_user(new_user: NewUser) -> Result<UserDto, diesel::result::Error> {
    let conn = establish_connection();

    let new_user = NewUser {
        email: new_user.email,
        hash: &digest(new_user.hash).to_string(),
        username: new_user.username,
        avatar_url: new_user.avatar_url,
    };

    insert_into(user::table)
    .values(new_user)
    .get_result::<UserDto>(&conn)
}

pub fn get_all() -> Result<Vec<UserDto>, diesel::result::Error> {
    let conn = establish_connection();

    user::table.get_results::<UserDto>(&conn)
}

pub fn find_user(user_id: uuid::Uuid) -> Result<UserDto, diesel::result::Error> {
    let conn = establish_connection();
    user::table.filter(id.eq_all(user_id)).first::<UserDto>(&conn)
}

pub fn find_user_by_email(user_email: String) -> Result<UserDto, diesel::result::Error> {
    let conn = establish_connection();

    user::table.filter(email.eq_all(user_email)).first::<UserDto>(&conn)
}