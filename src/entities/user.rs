use std::borrow::Cow;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User<'a> {
    pub id: Option<Uuid>,
    pub email: Cow<'a, str>
}