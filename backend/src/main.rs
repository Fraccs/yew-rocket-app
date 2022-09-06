use rocket::{routes, launch};
use database::db;
use routes::users;

pub mod database;
pub mod routes;

#[launch]
fn rocket() -> _ {
    let _db_connection = db::establish_connection();

    rocket::build().mount("/users", routes![users::login, users::register])
}
