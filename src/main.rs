use crate::migrator::Migrator;
use rocket::{catchers, routes};
use rocket_dyn_templates::Template;
use sea_orm::{entity::prelude::*, ConnectionTrait, DatabaseConnection, SqlxPostgresConnector};
use sea_orm_migration::MigratorTrait;
use sqlx::PgPool;

mod controllers;
mod entities;
mod migrator;
mod repositories;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let conn: DatabaseConnection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);

    Migrator::up(&conn, None).await.unwrap();
    let rocket = rocket::build()
        .manage(conn)
        .mount(
            "/",
            routes![
                controllers::home::index,
                controllers::home::about,
                controllers::home::url_shortener
            ],
        )
        .mount(
            "/contacts",
            routes![
                controllers::contact::contact_list,
                controllers::contact::new_contact,
                controllers::contact::save_contact,
                controllers::contact::delete_contact,
            ],
        )
        .mount(
            "/api/bakery",
            routes![
                controllers::bakery::create,
                controllers::bakery::delete,
                controllers::bakery::show,
                controllers::bakery::index,
            ],
        )
        .mount("/public", rocket::fs::FileServer::from("static/"))
        .register("/", catchers![controllers::home::not_found])
        .attach(Template::fairing());

    Ok(rocket.into())
}
