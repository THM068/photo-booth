use crate::controllers::views::{click_to_edit_form_view, click_to_edit_view};
use crate::model::Contact;
use crate::Contact_Db;
use rocket::{get, State};
use rocket_dyn_templates::{context, Template};
use sea_orm::DatabaseConnection;
use std::sync::{Arc, Mutex};

#[get("/click-to-edit")]
pub async fn click_to_edit(contact_state: &State<Contact_Db>) -> Template {
    let db = contact_state as &Contact_Db;
    let mut contact_mutex = db.lock().unwrap();
    let contact = contact_mutex.get("1");
    if let Some(contact_value) = contact {
        return Template::render(
            click_to_edit_view,
            context! {
                contact: contact_value
            },
        );
    } else {
        return Template::render(
            click_to_edit_view,
            context! {
                    contact: Contact::new("John".to_string(),
                    "Doe".to_string(), "email@example.com".to_string())

            },
        );
    }
}

#[get("/click-to-edit/<id>")]
pub async fn click_to_edit_id(id: i32, contact_state: &State<Contact_Db>) -> Template {
    let db = contact_state as &Contact_Db;
    let mut contact_mutex = db.lock().unwrap();
    let contact_option = contact_mutex.get(&id.to_string());
    let contact = match contact_option {
        Some(contact) => contact,
        None => &Contact::new("John".to_string(), "Doe".to_string(), "email@example.com".to_string())
    };

    Template::render(
        click_to_edit_form_view,
        context! {
            contact: contact
        },
    )
}
