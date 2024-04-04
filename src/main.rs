use rocket::routes;
use rocket_dyn_templates::Template;
use sea_orm::{
    ConnectionTrait, entity::prelude::*,
    SqlxPostgresConnector,
};
use sqlx::PgPool;

mod controllers;
mod entities;
mod repositories;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let conn = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    let rocket =
        rocket::build()
            .manage(conn)
            .mount("/", routes![
                controllers::home::index,
                controllers::home::contact,
                controllers::home::about
            ])
            .mount("/api/bakery", routes![
                controllers::bakery::create,
                controllers::bakery::delete,
                controllers::bakery::show,
                controllers::bakery::index,
            ])
            .mount("/public", rocket::fs::FileServer::from("static/"))
            .attach(Template::fairing());

    Ok(rocket.into())
}
