use std::borrow::Cow;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::uuid::Uuid;

use super::user::User;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Post<'a> {
    pub id: Option<Uuid>,
    pub title: Cow<'a, str>,
    pub text: Cow<'a, str>,
    pub author: User<'a>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Posts<'a> {
    pub posts: Vec<Post<'a>>
}
