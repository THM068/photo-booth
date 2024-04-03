use sea_orm::{
    entity::prelude::*, ActiveValue, ConnectOptions, ConnectionTrait, Database, DatabaseConnection,
    SqlxPostgresConnector, SqlxPostgresPoolConnection,
};

use rocket::{ routes};
use rocket_dyn_templates::{Template};
use sqlx::PgPool;
mod controllers;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let rocket =
        rocket::build()
            .mount("/", routes![
                controllers::home::index,
                controllers::home::contact])
            .mount("/public", rocket::fs::FileServer::from("static/"))
            .attach(Template::fairing());

    Ok(rocket.into())
}
