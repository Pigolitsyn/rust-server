#[macro_use] extern crate rocket;

mod entities;
mod controllers;
use rocket::fs::{FileServer, relative, Options};

#[launch]
fn rocket() -> _ {
    rocket::build().attach(controllers::post_controller::post_controller())
    .mount("/", FileServer::new(relative!("static"), Options::Index).rank(30))
}
