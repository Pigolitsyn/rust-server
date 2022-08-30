use crate::schema::user;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hash: String
}


#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub hash: &'a str
}