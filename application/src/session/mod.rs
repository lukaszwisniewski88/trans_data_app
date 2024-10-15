use std::time::SystemTime;

use diesel::prelude::*;
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
    serde::Serialize,
    State,
};
use uuid::Uuid;

use crate::Postgres;
#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub id: String,
    pub user_id: Option<Uuid>,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let session_id = request.cookies().get("session_id");
        let mut conn = infrastructure::establish_connection();
        match session_id {
            Some(session_id) => {
                let id = session_id.value().to_string();
                if let Ok(session) = get_session(id, &mut conn) {
                    Outcome::Success(session)
                } else {
                    Outcome::Error((Status::Unauthorized, ()))
                }
            }
            None => {
                if let Ok(session) = create_session(None, &mut conn) {
                    Outcome::Success(session)
                } else {
                    Outcome::Error((Status::Unauthorized, ()))
                }
            }
        }
    }
}
pub fn create_session(
    user_id: Option<Uuid>,
    db: &mut PgConnection,
) -> Result<Session, diesel::result::Error> {
    use domain::schema::sessions;

    let id = shared::ids::generate_id(15);
    let new_session = domain::models::NewSession { id, user_id };
    match diesel::insert_into(sessions::table)
        .values(&new_session)
        .get_result::<Session>(db)
    {
        Ok(session) => Ok(session),
        Err(e) => Err(e),
    }
}

pub fn get_session(
    session_id: String,
    conn: &mut PgConnection,
) -> Result<Session, diesel::result::Error> {
    use domain::schema::sessions::dsl::*;

    sessions.filter(id.eq(session_id)).first::<Session>(conn)
}
