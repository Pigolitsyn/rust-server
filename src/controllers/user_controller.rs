use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

use crate::{services::user_service::{find_user, create_user, get_all, find_user_by_email}, entities::user::{NewUser, UserDto}, utilities::convert_rocket_uuid_to_uuid};

#[post("/create-user", data = "<user>")]
async fn post_user(user: Json<NewUser<'_>>) {
    let user = user.0;
    create_user(user);
}

#[get("/get-user-by-id/<id>")]
async fn get_user(id: Uuid) -> Result<Json<UserDto>, String> {
    let converted_id = convert_rocket_uuid_to_uuid(id);
    match find_user(converted_id) {
        Ok(user) => return Ok(Json(user)),
        Err(err) => return Err(err.to_string())
    }
}

#[get("/get-user-by-email/<email>")]
async fn get_user_by_email(email: String) -> Result<Json<UserDto>, String> {
    match find_user_by_email(email) {
        Ok(user) => return Ok(Json(user)),
        Err(err) => return Err(err.to_string())
    }
}

#[get("/get-users")]
async fn get_all_users() -> Result<Json<Vec<UserDto>>, String> {
    match get_all() {
        Ok(user) => return Ok(Json(user)),
        Err(err) => return Err(err.to_string())
    }
}


pub fn user_controller() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("user_controller", |rocket| async {
        rocket.mount("/user-controller", routes![get_user, post_user, get_all_users, get_user_by_email])
    })
}