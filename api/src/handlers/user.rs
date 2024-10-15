use application::session::Session;
use application::Postgres;
use domain::models::{NewUser, User};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};
use uuid::Uuid;
use validator::Validate;

#[get("/")]
pub async fn list_users(session: Session, db: Postgres) -> Result<Json<Vec<User>>, Status> {
    println!("Session: {:?}", session);
    // match application::users::read::list_users() {
    //     Ok(users) => Ok(Json(users)),
    //     Err(_) => Err(Status::InternalServerError),
    // }
    match db.run(|c| application::users::read::list_users(c)).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<id>")]
pub async fn get_user(id: String, db: Postgres) -> Result<Json<User>, Status> {
    // match application::users::read::list_user(Uuid::parse_str(id).unwrap()) {
    //     Ok(user) => Ok(Json(user)),
    //     Err(_) => Err(Status::NotFound),
    // }
    match db.run(|c| application::users::read::list_user(id, c)).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/", data = "<new_user>")]
pub async fn create_user(new_user: Form<NewUser>, db: Postgres) -> Result<Json<User>, Status> {
    if let Err(e) = new_user.validate() {
        eprintln!("Validation error: {:?}", e);
        return Err(Status::UnprocessableEntity);
    }
    let data = db
        .run(|c| application::users::create::create_user(new_user.into_inner(), c))
        .await;
    match data {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
