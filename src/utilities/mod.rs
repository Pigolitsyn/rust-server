pub fn convert_rocket_uuid_to_uuid(source: rocket::serde::uuid::Uuid) -> uuid::Uuid {
    match uuid::Uuid::parse_str(&source.to_string()) {
        Ok(source) => return source,
        Err(err) => panic!("{} was happened", err)
    }
}