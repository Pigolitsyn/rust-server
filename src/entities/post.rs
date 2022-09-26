use std::borrow::Cow;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::post;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Post<'a> {
    pub id: uuid::Uuid,
    pub title: Cow<'a, str>,
    pub text: Cow<'a, str>,
    pub author_id: uuid::Uuid
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name="post"]
#[serde(crate = "rocket::serde")]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub text: &'a str,
    pub author_id: uuid::Uuid,
}