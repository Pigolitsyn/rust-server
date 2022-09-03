use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

use crate::{services::user_service::{find_user, create_user}, entities::user::{NewUser, UserDto}, utilities::convert_rocket_uuid_to_uuid};

#[post("/create-user", data = "<user>")]
async fn post_user(user: Json<NewUser<'_>>) {
    let user = user.0;
    create_user(user);
}

#[get("/get-user/<id>")]
async fn get_user(id: Uuid) -> Result<Json<UserDto>, String> {
    let converted_id = convert_rocket_uuid_to_uuid(id);
    match find_user(converted_id) {
        Ok(user) => return Ok(Json(user)),
        Err(err) => return Err(err.to_string())
    }
}

pub fn user_controller() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("user_controller", |rocket| async {
        rocket.mount("/user-controller", routes![get_user, post_user])
    })
}