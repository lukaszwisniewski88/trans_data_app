use crate::schema::sessions;
use crate::schema::users;
use diesel::prelude::*;
use rocket::{serde::Serialize, FromForm};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Queryable, Ord, Eq, PartialEq, PartialOrd)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

#[derive(Insertable, Validate, FromForm)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub id: String,
    pub user_id: Option<Uuid>,
}
