use rocket::serde::json::Json;

use crate::entities::{post::{Post}, user::User};

#[get("/get-user/<id>")]
async fn get_posts() -> Json<Vec<Post<'static>>> {
    
}

pub fn user_controller() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("user_controller", |rocket| async {
        rocket.mount("/user-controller", routes![get_posts])
    })
}