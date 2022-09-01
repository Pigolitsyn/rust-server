use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

use crate::{services::user_service::find_user, entities::user::User};

fn convert_rocket_uuid_to_uuid(source: rocket::serde::uuid::Uuid) -> uuid::Uuid {
    match uuid::Uuid::parse_str(&source.to_string()) {
        Ok(source) => return source,
        Err(err) => panic!("{} was happened", err)
    }
}

#[get("/get-user/<id>")]
async fn get_user(id: Uuid) -> Result<Json<User>, String> {
    let converted_id = convert_rocket_uuid_to_uuid(id);
    match find_user(converted_id) {
        Ok(user) => return Ok(Json(user)),
        Err(err) => return Err(err.to_string())
    }
}

pub fn user_controller() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("user_controller", |rocket| async {
        rocket.mount("/user-controller", routes![get_user])
    })
}

pub fn auth_controller() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_request("user_controller", |req, _| Box::pin(async {
        
    }))
}
