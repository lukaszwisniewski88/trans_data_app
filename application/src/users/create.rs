use diesel::prelude::*;
use domain::models::{NewUser, User};
use std::io::Error;

pub fn create_user(new_user: NewUser, db: &mut PgConnection) -> Result<User, Error> {
    use domain::schema::users;

    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(db)
    {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::new(std::io::ErrorKind::Other, "Error creating user")),
    }
}
