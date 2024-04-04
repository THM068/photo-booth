use rocket::{delete, get, post, put, State};
use rocket::http::hyper::Response;
use sea_orm::DatabaseConnection;
use crate::entities::{prelude::*, bakery::*, bakery};
use sea_orm::*;

#[post("/")]
pub async fn create(db: &State<DatabaseConnection>) -> String {
    let db = db as &DatabaseConnection;
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(db).await;
    match res {
        Ok(r) => println!("Bakery created {:?}", r),
        Err(err) => println!("Error creating bakery: {:?}", err)
    }
    "create".to_string()
}
#[get("/<id>")]
pub async  fn show(id: u32) -> String {
    todo!()
}

#[delete("/<id>")]
pub async  fn delete(id: u32) -> String {
    todo!()
}

#[get("/")]
pub async fn index( db: &State<DatabaseConnection>) -> String {

    let db = db as &DatabaseConnection;
    let bakeries = Bakery::find().all(db).await;
    match  bakeries {
        Ok(result) => println!("{:?}", result),
        Err(err) => println!("Error fetching bakeries: {:?}", err)
    }

    "grt".to_string()
}

// #[put("/<id>")]
// pub async  fn update() -> String {
//     todo!()
// }