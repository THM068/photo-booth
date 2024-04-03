use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/contact")]
pub fn contact() -> Template {
    Template::render("contact", context! {
        title: "Contact"
    })
}