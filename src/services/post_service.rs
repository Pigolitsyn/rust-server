use diesel::insert_into;
use crate::entities::post::NewPost;
use crate::schema::post;
use crate::diesel::RunQueryDsl;

use crate::{entities::post::Post, connection::establish_connection};

pub fn create_post<'a>(new_post: NewPost<'a>) {
    let conn = establish_connection();

}