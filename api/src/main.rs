#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;
use tracing_fairing::{LogLevel, LogType};

use routes::{create_user, delete_user, get_user, get_users, update_user};

mod db;
mod error;
mod prelude;
mod routes;
mod tracing_fairing;
mod user;

#[launch]
fn launch() -> _ {
    tracing_fairing::init_tracing(LogLevel::Normal, LogType::Json);

    rocket::build()
        .attach(db::pool::UsersDb::init())
        .attach(tracing_fairing::TracingFairing)
        .register("/", catchers![error::handle_error])
        .mount(
            "/api/v1/",
            routes![get_user, create_user, get_users, update_user, delete_user],
        )
}
