#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate diesel;

mod services;
mod connection;
mod entities;
mod controllers;
use rocket::{fs::{FileServer, relative, Options}};
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(controllers::post_controller::post_controller())
        .attach(controllers::user_controller::user_controller())
    .mount("/", FileServer::new(relative!("static"), Options::Index).rank(30))
}
