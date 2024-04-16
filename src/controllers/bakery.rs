use rocket::{delete, FromForm, get, post, put, State, uri};
use rocket::form::Form;
use rocket::http::hyper::Response;
use sea_orm::DatabaseConnection;
use crate::entities::{prelude::*, bakery::*, bakery};
use sea_orm::*;
use crate::repositories::bakeryRepository::{BakeryRepository, BakeryRepositoryImpl};
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::Redirect;
use rocket::serde::{Serialize};
use rocket_dyn_templates::{context, Template};

const LIST_BAKERIES_VIEWS: &str = "bakeries";

#[post("/", data = "<bakery>")]
pub async fn create(db: &State<DatabaseConnection>, bakery: Form<BakeryDTO>) -> Redirect {
    let bakery = bakery.into_inner();
    let db = db as &DatabaseConnection;
    // let bakeryRepo = BakeryRepositoryImpl {};
    // let res = bakeryRepo.create_bakery(db, ).await;
    // match res {
    //     Ok(r) => println!("Bakery created {:?}", r),
    //     Err(err) => println!("Error creating bakery: {:?}", err)
    // }
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set(bakery.name.to_owned()),
        profit_margin: ActiveValue::Set(bakery.profit_margin),
        ..Default::default()
    };
    let result = Bakery::insert(happy_bakery).exec(db).await;
    match result {
        Ok(result) => {
            Redirect::to(uri!("/api/bakery"))
        },
        Err(err) => {
            Redirect::to(uri!("/api/bakery"))
        }
    }

}

#[get("/<id>")]
pub async fn show(id: u32) -> String {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> String {
    todo!()
}

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>) -> Template {
    let db = db as &DatabaseConnection;
    let bakeries = Bakery::find().all(db).await;

    match bakeries {
        Ok(result) => {
            let bakery_list = result.iter().map(|b| {
                BakeryDTO{
                    name: b.name.to_owned(),
                    profit_margin: b.profit_margin
                }
            }).collect::<Vec<_>>();
            Template::render(LIST_BAKERIES_VIEWS, context! {
                bakeries: bakery_list
                })
        }
        Err(err) => {
            println!("Failed");
            Template::render(LIST_BAKERIES_VIEWS, context! {

                 error: "Error fetching bakeries"
                })
        }
    }
}

#[derive(Serialize, Debug, FromForm)]
struct BakeryDTO {
    name: String,
    profit_margin: f64,
}
