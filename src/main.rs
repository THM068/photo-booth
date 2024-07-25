use crate::migrator::Migrator;
use crate::model::Contact;
use rocket::{catchers, routes};
use rocket_dyn_templates::Template;
use sea_orm::{entity::prelude::*, ConnectionTrait, DatabaseConnection, SqlxPostgresConnector};
use sea_orm_migration::MigratorTrait;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod controllers;
mod entities;
mod migrator;
mod model;
mod repositories;

type Contact_Db = Arc<Mutex<HashMap<String, Contact>>>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    let conn: DatabaseConnection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    let mut contact_db = HashMap::new();
    contact_db.insert(
        "1".to_string(),
        Contact::new(
            "Jonny J".to_string(),
            "Mafela".to_string(),
            "thomas@examples.com".to_string(),
        ),
    );
    let contact_db: Contact_Db = Arc::new(Mutex::new(contact_db));

    Migrator::up(&conn, None).await.unwrap();
    let rocket = rocket::build()
        .manage(conn)
        .manage(contact_db.clone())
        .mount(
            "/",
            routes![
                controllers::home::streamer,
                controllers::home::index,
                controllers::home::about,
                controllers::home::url_shortener,
                controllers::authentication::login,
                controllers::authentication::authenticate,
                controllers::register::sign_up,
                controllers::register::register,
            ],
        )
        .mount(
            "/contacts",
            routes![
                controllers::contact::contact_list,
                controllers::contact::new_contact,
                controllers::contact::save_contact,
                controllers::contact::delete_contact,
                controllers::contact::show_contact,
                controllers::contact::work_cation,
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
        .mount("/examples",
               routes![
                   controllers::examples::click_to_edit,
                   controllers::examples::click_to_edit_id,
               ])
        .mount("/public", rocket::fs::FileServer::from("static/"))
        .register("/", catchers![controllers::home::not_found])
        .attach(Template::fairing());

    Ok(rocket.into())
}
