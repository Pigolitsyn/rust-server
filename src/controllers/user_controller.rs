use crate::{entities::{user::User}, services::user_service::find_user};

#[get("/get-user/<id>")]
async fn get_user(id: rocket::serde::uuid::Uuid) -> String {
    let res = uuid::Uuid::parse_str(&id.to_string());
    match res {
        Ok(id) => return find_user(id).email,
        Err(_) => return "Can't parse uuid".to_string()
    }
    
}

pub fn user_controller() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("user_controller", |rocket| async {
        rocket.mount("/user-controller", routes![get_user])
    })
}