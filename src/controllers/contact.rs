use crate::entities::{contact, contact::*, prelude::*};
use rocket::form::{Contextual, Form};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{delete, get, post, State};
use rocket_dyn_templates::{context, Template};
use sea_orm::*;

use crate::repositories::{Contact_DTO, Contact_Entity};

#[get("/<id>")]
pub async fn show_contact(id: i32, db: &State<DatabaseConnection>) -> Template {
    let db = db as &DatabaseConnection;
    let contact_result = Contact::find_by_id(id).one(db).await;

    match contact_result {
        Ok(contactOption) => match contactOption {
            Some(contact) => {
                println!("Contact found {:?}", contact.id);
                let contact_entity = Contact_Entity {
                    id: contact.id,
                    given_name: contact.given_name.to_owned(),
                    family_name: contact.family_name.to_owned(),
                    phone: contact.phone.to_owned(),
                    email: contact.email.to_owned(),
                };
                println!("Contact found {:?}", contact_entity.given_name);
                return Template::render(
                    "contact_show",
                    context! {
                        title: "Contact",
                        contact: contact_entity
                    },
                );
            }
            None => {
                println!("Contact not found");
                Template::render(
                    "contact_show",
                    context! {
                        title: "Contact",
                        message: "Contact not found."
                    },
                )
            }
        },
        Err(error) => {
            println!("Contact not found 2");
            Template::render(
                "contact_show",
                context! {
                    title: "Contact",
                    message: "Error loading contact."
                },
            )
        }
    }
}

#[get("/?<q>")]
pub async fn contact_list(
    db: &State<DatabaseConnection>,
    flash: Option<FlashMessage<'_>>,
    q: Option<String>,
) -> Template {
    let message = flash.map_or_else(|| String::default(), |msg| msg.message().to_string());
    let db = db as &DatabaseConnection;
    let contacts = match q {
        Some(query) => {
            Contact::find()
                .filter(contact::Column::GivenName.contains(query))
                .all(db)
                .await
        }
        None => Contact::find().all(db).await,
    };

    if let Ok(result) = contacts {
        let contact_list = result
            .iter()
            .map(|c| Contact_Entity {
                id: c.id,
                given_name: c.given_name.to_owned(),
                family_name: c.family_name.to_owned(),
                phone: c.phone.to_owned(),
                email: c.email.to_owned(),
            })
            .collect::<Vec<_>>();

        return Template::render(
            "contact",
            context! {
                title: "Contact",
                message: message,
                contacts: contact_list
            },
        );
    }

    Template::render(
        "contact",
        context! {
                    title: "Contact",
                    message: message,
        contacts: Vec::<Contact_Entity>::new()
                            },
    )
}

#[get("/new")]
pub fn new_contact() -> Template {
    Template::render(
        "new_contact",
        context! {
            title: "New Contact"
        },
    )
}
#[delete("/<id>")]
pub async fn delete_contact(id: i32, db: &State<DatabaseConnection>) -> String {
    let db = db as &DatabaseConnection;
    let delete_result = Contact::delete_by_id(id).exec(db).await;

    match delete_result {
        Ok(_) => "".to_string(),
        Err(_) => "".to_string(),
    }
}
#[post("/save", data = "<contact_form>")]
pub async fn save_contact(
    db: &State<DatabaseConnection>,
    contact_form: Form<Contextual<'_, Contact_DTO>>,
) -> Result<Flash<Redirect>, Template> {
    let db = db as &DatabaseConnection;
    if let Some(ref contact_dto) = contact_form.value {
        let contact_active_model = contact::ActiveModel {
            given_name: ActiveValue::Set(contact_dto.given_name.to_owned()),
            family_name: ActiveValue::Set(contact_dto.family_name.to_owned()),
            phone: ActiveValue::Set(contact_dto.phone.to_owned()),
            email: ActiveValue::Set(contact_dto.email.to_owned()),
            ..Default::default()
        };
        let result = Contact::insert(contact_active_model).exec(db).await;
        match result {
            Ok(result) => {
                println!("Contact saved {:?}", result.last_insert_id as i32);
                let message =
                    Flash::success(Redirect::to("/contacts"), "Contact saved successfully.");
                return Ok(message);
            }
            Err(err) => {
                return Err(Template::render(
                    "new_contact",
                    context! {
                        title: "New Contact",
                        error: "Error saving contact."
                    },
                ));
            }
        }
    }
    Err(Template::render(
        "new_contact",
        context! {
            title: "New Contact",
            error: "Error saving contact."
        },
    ))
}
#[get("/work-cation")]
pub async fn work_cation() -> Template {
    Template::render(
        "work_cation",
        context! {
            title: "Work-cation"
        },
    )
}
