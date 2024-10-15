use std::io::Error;

use diesel::prelude::*;
use domain::models::User;
use rocket::response::status::NotFound;
use uuid::Uuid;

pub fn list_user(user_id: String, db: &mut PgConnection) -> Result<User, NotFound<String>> {
    use domain::schema::users;
    let uuid = Uuid::parse_str(&user_id).unwrap();
    match users::table::find(users::table, uuid).first::<User>(db) {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => Err(NotFound("User not found".to_string())),
            _ => Err(NotFound("User not found due to db error".to_string())),
        },
    }
}

pub fn list_users(db: &mut PgConnection) -> Result<Vec<User>, Error> {
    use domain::schema::users;

    match users::table.select(users::all_columns).load::<User>(db) {
        Ok(users) => Ok(users),
        Err(_) => Err(Error::new(
            std::io::ErrorKind::Other,
            "Error fetching users",
        )),
    }
}
