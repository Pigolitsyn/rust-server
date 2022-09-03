use rocket::serde::json::Json;

use crate::entities::{post::{Post}};

#[get("/get-all")]
async fn get_posts() -> Json<Vec<Post<'static>>> {
    Json(vec![])
}

pub fn post_controller() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("post_controller", |rocket| async {
        rocket.mount("/post-controller", routes![get_posts])
    })
}