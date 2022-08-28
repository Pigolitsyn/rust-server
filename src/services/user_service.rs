use crate::connection::{establish_connection};
use crate::entities::user::{NewUser};
use crate::diesel::insert_into;
use diesel::{RunQueryDsl};

pub fn create_user() {
    use crate::schema::user;
    let conn = establish_connection();
    insert_into(user::table)
        .default_values()
        .execute(&conn);
}