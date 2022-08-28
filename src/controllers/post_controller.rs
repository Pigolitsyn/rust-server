use rocket::serde::json::Json;

use crate::entities::{post::{Posts, Post}, user::User};

#[get("/get-all")]
async fn get_posts() -> Json<Posts<'static>> {
    let user = User {
        id: Some("7f205202-7ba1-4c39-b2fc-3e630722bf9f".parse().unwrap()),
        email: std::borrow::Cow::Borrowed("hello_world@email.com"),
    };

    let post = Post {
        id: Some("7f205202-7ba1-4c39-b2fc-3e630722bf9f".parse().unwrap()),
        title: std::borrow::Cow::Borrowed("hello_world@email.com"),
        text: std::borrow::Cow::Borrowed("hello_world@email.com"),
        author: user,
    };
    
    let user = User {
        id: Some("7f205202-7ba1-4c39-b2fc-3e630722bf9f".parse().unwrap()),
        email: std::borrow::Cow::Borrowed("hello_world@email.com"),
    };

    let post2 = Post {
        id: Some("7f205202-7ba1-4c39-b2fc-3e630722bf9f".parse().unwrap()),
        title: std::borrow::Cow::Borrowed("hello_world@email.com"),
        text: std::borrow::Cow::Borrowed("hello_world@email.com"),
        author: user,
    };

    let posts = Posts { posts: vec![post, post2] };

    Json(posts)
}

pub fn post_controller() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("post_controller", |rocket| async {
        rocket.mount("/post-controller", routes![get_posts])
    })
}