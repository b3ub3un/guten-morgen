//add the modules
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

//add imports below
use api::user_api::create_user;
use repository::mongodb_repo::MongoRepo;

use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Welcome to the API")))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/", routes![create_user])
}
