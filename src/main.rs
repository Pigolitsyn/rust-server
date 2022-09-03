#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate diesel;

mod services;
mod connection;
mod entities;
mod controllers;
use rocket::{fs::{FileServer, relative, Options}, http::CookieJar};
mod schema;
mod utilities;

struct Cookies(CookieJar<'static>);

#[launch]
fn rocket() -> _ {
    rocket::build().manage(Cookies)
        .attach(controllers::post_controller::post_controller())
        .attach(controllers::user_controller::user_controller())
        .attach(controllers::images_controller::images_controller())
    .mount("/", FileServer::new(relative!("static"), Options::Index).rank(30))
}
