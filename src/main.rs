
use crate::controllers::home::contact;
use rocket::{ routes};
use rocket_dyn_templates::{Template};

mod controllers;





#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket =
        rocket::build()
            .mount("/", routes![
                controllers::home::index,
                controllers::home::contact])
            .mount("/public", rocket::fs::FileServer::from("static/"))
            .attach(Template::fairing());

    Ok(rocket.into())
}
