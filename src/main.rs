#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate diesel;

mod services;
mod connection;
mod entities;
mod controllers;
use entities::user::User;
use rocket::{fs::{FileServer, relative, Options}, fairing::AdHoc, http::CookieJar, State};
mod schema;

struct Cookies(CookieJar<'static>);

#[launch]
fn rocket() -> _ {
    rocket::build().manage(Cookies)
        .attach(controllers::post_controller::post_controller())
        .attach(controllers::user_controller::user_controller())
        .attach(AdHoc::on_request("Put Rewriter", |req, _| Box::pin(async move {
            req.cookies();
            State
        })))
    .mount("/", FileServer::new(relative!("static"), Options::Index).rank(30))
}
