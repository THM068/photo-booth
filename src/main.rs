use rocket::{catchers, routes};
use rocket_dyn_templates::Template;
use sea_orm::{
    ConnectionTrait, entity::prelude::*,
    SqlxPostgresConnector, DatabaseConnection
};
use sqlx::PgPool;
use sea_orm_migration::MigratorTrait;
use crate::migrator::Migrator;

mod controllers;
mod entities;
mod repositories;
mod migrator;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let conn: DatabaseConnection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);

    Migrator::up(&conn, None).await.unwrap();
    let rocket =
        rocket::build()
            .manage(conn)
            .mount("/", routes![
                controllers::home::index,
                controllers::home::contact,
                controllers::home::about,
                controllers::home::url_shortener
            ])
            .mount("/api/bakery", routes![
                controllers::bakery::create,
                controllers::bakery::delete,
                controllers::bakery::show,
                controllers::bakery::index,
            ])
            .mount("/public", rocket::fs::FileServer::from("static/"))
            .register("/", catchers![controllers::home::not_found   ])
            .attach(Template::fairing());

    Ok(rocket.into())
}
