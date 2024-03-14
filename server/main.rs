#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};

use std::{io::ErrorKind, sync::Arc};
use surrealdb::{sql::Object, Datastore, Session};

use crate::db::{AffectedRows, DB};

use cors::*;

mod cors;
mod db;
mod error;
mod prelude;
mod utils;

#[launch]
async fn rocket() -> _ {
    let ds = Arc::new(Datastore::new("memory").await.unwrap());
    let sesh = Session::for_db("my_ns", "my_db");

    let db = DB { ds, sesh };

    rocket::build()
        .mount(
            "/",
            routes![add_task, get_task, get_all_tasks, toggle_task, delete_task],
        )
        .attach(CORS)
        .manage(db)
}