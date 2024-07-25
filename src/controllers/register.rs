use crate::controllers::User_Form;
use crate::entities::{prelude::*, user};
use bcrypt::{hash, DEFAULT_COST};
use rocket::form::{Contextual, Form};
use rocket::response::{Flash, Redirect, Responder};
use rocket::{get, post, uri, State};
use rocket_dyn_templates::{context, Template};
use sea_orm::*;
use std::task::Context;

#[get("/sign-up")]
pub fn sign_up() -> Template {
    Template::render(
        "signup",
        context! {
            title: "Sign Up"
        },
    )
}

#[post("/register", data = "<user_form>")]
pub async fn register<'r>(
    db: &State<DatabaseConnection>,
    user_form: Form<Contextual<'_, User_Form<'r>>>,
) -> Result<Flash<Redirect>, Template> {
    let db = db as &DatabaseConnection;
    if let Some(ref user_form_payload) = user_form.value {
        let user: Result<Option<user::Model>, DbErr> = User::find()
            .filter(user::Column::Email.eq(user_form_payload.email))
            .one(db)
            .await;

        if let Ok(Some(_)) = user {
            return Err(Template::render(
                "signup",
                context! {
                    title: "Sign Up",
                    errors: vec!["Email already exists".to_string()]
                },
            ));
        }

        let user_active_model = user::ActiveModel {
            email: ActiveValue::Set(user_form_payload.email.to_owned()),
            password: ActiveValue::Set(hash(&user_form_payload.password, DEFAULT_COST).unwrap()),
            ..Default::default()
        };
        let user_result = user::Entity::insert(user_active_model).exec(db).await;

        match user_result {
            Ok(_) => {
                let message =
                    Flash::success(Redirect::to(uri!("/login")), "User registered successfully");
                return Ok(message);
            }
            Err(error) => {
                return Err(Template::render(
                    "signup",
                    context! {
                        title: "Sign Up",
                        errors: vec![error.to_string()]
                    },
                ));
            }
        }
    }

    let error_messages: Vec<String> = user_form
        .context
        .errors()
        .map(|error| {
            let name = error.name.as_ref().unwrap().to_string();
            let description = error.to_string();
            format!("'{}' {}", name, description)
        })
        .collect();
    Err(Template::render(
        "signup",
        context! {
            title: "Sign Up",
            errors: error_messages
        },
    ))
}
