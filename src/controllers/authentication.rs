use crate::repositories::Contact_DTO;
use rocket::form::{Contextual, Form};
use rocket::response::{Redirect, Responder};
use rocket::{get, post, FromForm};
use rocket_dyn_templates::{context, Template};

#[derive(FromForm, Debug)]
struct RequestSignIn {
    #[field(validate=len(1..))]
    email: String,
    #[field(validate=len(1..))]
    password: String,
}
#[get("/login")]
pub async fn login() -> Template {
    Template::render(
        "login",
        context! {
            title: "Login"
        },
    )
}

#[post("/sign-in", data = "<req_sign_in>")]
pub async fn authenticate(
    req_sign_in: Form<Contextual<'_, RequestSignIn>>,
) -> Result<Redirect, Template> {
    if let Some(ref sign_request) = req_sign_in.value {
        println!("Email: {}", sign_request.email);
        println!("Password: {}", sign_request.password);
        return Ok(Redirect::to("/"));
    }
    let error_messages: Vec<String> = req_sign_in
        .context
        .errors()
        .map(|error| {
            let name = error.name.as_ref().unwrap().to_string();
            let description = error.to_string();
            format!("'{}' {}", name, description)
        })
        .collect();

    Err(Template::render(
        "login",
        context! {
            title: "Login",
            errors: error_messages
        },
    ))
}
