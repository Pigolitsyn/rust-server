#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate diesel;
#[macro_use]
extern crate dotenv;

mod services;
mod connection;
mod entities;
mod controllers;
use rocket::fs::{FileServer, relative, Options};
use services::user_service::create_user;
mod schema;

#[launch]
fn rocket() -> _ {
    create_user();
    rocket::build().attach(controllers::post_controller::post_controller())
    .mount("/", FileServer::new(relative!("static"), Options::Index).rank(30))
}
