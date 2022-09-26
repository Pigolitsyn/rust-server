use crate::schema::user;
use rocket::serde::{Serialize, Deserialize};

// to create
#[derive(Insertable, Serialize, Deserialize)]
#[table_name="user"]
#[serde(crate = "rocket::serde")]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub hash: &'a str,
    pub username: &'a str,
    pub avatar_url: &'a str
}

// to login
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserIdentity {
    pub email: String,
    pub password: String
}

// return to front
#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user"]
pub struct UserDto {
    pub id: uuid::Uuid,
    pub email: String,
    pub hash: String,
    pub username: String,
    pub avatar_url: String,
}