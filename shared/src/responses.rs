use domain::models::User;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    User(User),
    Users(Vec<User>),
    Message(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub status: u16,
    pub body: ResponseBody,
}
