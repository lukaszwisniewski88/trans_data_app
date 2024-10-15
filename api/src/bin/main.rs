#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;

use api::handlers::user;
use application::Postgres;
use rocket::fs::{relative, FileServer, Options};

#[launch]
fn rocket() -> _ {
    let web_options: Options = Options::NormalizeDirs | Options::Index;
    rocket::build()
        .attach(Postgres::fairing())
        .mount(
            "/api/users",
            routes![user::list_users, user::get_user, user::create_user],
        )
        .mount("/", FileServer::new(relative!("../web/build"), web_options))
}
