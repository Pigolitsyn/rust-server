use std::borrow::Cow;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Post<'a> {
    pub id: Option<Uuid>,
    pub title: Cow<'a, str>,
    pub text: Cow<'a, str>,
    pub author_id: Option<Uuid>
}