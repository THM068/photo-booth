use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        title: "Home"
    })
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("about", context! {
        title: "About"
    })
}

#[get("/contact")]
pub fn contact() -> Template {
    Template::render("contact", context! {
        title: "Contact"
    })
}