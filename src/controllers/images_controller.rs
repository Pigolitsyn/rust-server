use rocket::fs::TempFile;

#[post("/upload", format = "image/jpeg", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    let id = uuid::Uuid::new_v4();
    let id = id.to_urn().to_string();
    file.persist_to("/home/pigolitsyn/Desktop/rust_blog/rust-server/static/photos/".to_owned() + &id[9..] + ".jpeg").await
}

pub fn images_controller() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("images-controller", |rocket| async {
        rocket.mount("/images-controller", routes![upload])
    })
}