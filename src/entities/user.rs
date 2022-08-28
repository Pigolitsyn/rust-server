use crate::schema::user;
use uuid::Uuid;

#[derive(Queryable)]
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

